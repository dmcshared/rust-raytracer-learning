use std::ops;

use crate::util::fuzzy_comparison::{f64_fuzzy_eq, FuzzyPartialEq};

use super::mix_modes::MixMode;

/// A color in RGBA format.
/// This is the most commonly used format for the color api.
///
/// It is composed of four parts:
/// - Red: the red component of the color [0.0, 1.0]
/// - Green: the green component of the color [0.0, 1.0]
/// - Blue: the blue component of the color [0.0, 1.0]
/// - Alpha: the alpha component of the color [0.0, 1.0]
///
/// Example:
/// ```
/// use raytracer::gfx::primitives::{color::ColorRGBA, mix_modes::MixMode};
///
/// let red = ColorRGBA::new(1., 0., 0., 1.0);
/// let green = ColorRGBA::new(0., 1., 0., 0.5);
///
/// let mixed = red.mix(green, MixMode::Alpha);
///
/// assert_eq!(mixed, ColorRGBA::new(0.5, 0.5, 0.0, 1.0));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ColorRGBA(pub f64, pub f64, pub f64, pub f64);

pub mod default_palettes {
    use super::ColorRGBA;
    pub mod full_bright {
        use super::ColorRGBA;

        pub const RED: ColorRGBA = ColorRGBA(1.0, 0.0, 0.0, 1.0);
        pub const YELLOW: ColorRGBA = ColorRGBA(1.0, 1.0, 0.0, 1.0);
        pub const GREEN: ColorRGBA = ColorRGBA(0.0, 1.0, 0.0, 1.0);
        pub const CYAN: ColorRGBA = ColorRGBA(0.0, 1.0, 1.0, 1.0);
        pub const BLUE: ColorRGBA = ColorRGBA(0.0, 0.0, 1.0, 1.0);
        pub const MAGENTA: ColorRGBA = ColorRGBA(1.0, 0.0, 1.0, 1.0);
        pub const WHITE: ColorRGBA = ColorRGBA(1.0, 1.0, 1.0, 1.0);
        pub const BLACK: ColorRGBA = ColorRGBA(0.0, 0.0, 0.0, 1.0);
    }
}

impl ColorRGBA {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> ColorRGBA {
        ColorRGBA(r, g, b, a)
    }

    pub fn invert(&self) -> ColorRGBA {
        ColorRGBA(1.0 - self.0, 1.0 - self.1, 1.0 - self.2, self.3)
    }

    pub fn mul_all(&self, other: f64) -> ColorRGBA {
        ColorRGBA(
            self.0 * other,
            self.1 * other,
            self.2 * other,
            self.3 * other,
        )
    }

    pub fn mix(&self, other: ColorRGBA, mode: MixMode) -> ColorRGBA {
        match mode {
            MixMode::Avg => ColorRGBA(
                (self.0 + other.0) * 0.5,
                (self.1 + other.1) * 0.5,
                (self.2 + other.2) * 0.5,
                (self.3 + other.3) * 0.5,
            ),
            MixMode::Alpha => ColorRGBA(
                (other.0 * other.3 + self.0 * self.3 * (1.0 - other.3))
                    / (other.3 + self.3 * (1.0 - other.3)),
                (other.1 * other.3 + self.1 * self.3 * (1.0 - other.3))
                    / (other.3 + self.3 * (1.0 - other.3)),
                (other.2 * other.3 + self.2 * self.3 * (1.0 - other.3))
                    / (other.3 + self.3 * (1.0 - other.3)),
                other.3 + self.3 * (1.0 - other.3),
            ),
            MixMode::Add => ColorRGBA(self.0 + other.0, self.1 + other.1, self.2 + other.2, self.3),
            MixMode::Sub => ColorRGBA(self.0 - other.0, self.1 - other.1, self.2 - other.2, self.3),
            MixMode::Mul => ColorRGBA(
                self.0 * other.0,
                self.1 * other.1,
                self.2 * other.2,
                self.3 * other.3,
            ),
            MixMode::Div => ColorRGBA(
                self.0 / other.0,
                self.1 / other.1,
                self.2 / other.2,
                self.3 / other.3,
            ),
            MixMode::Min => ColorRGBA(
                self.0.min(other.0),
                self.1.min(other.1),
                self.2.min(other.2),
                self.3.min(other.3),
            ),
            MixMode::Max => ColorRGBA(
                self.0.max(other.0),
                self.1.max(other.1),
                self.2.max(other.2),
                self.3.max(other.3),
            ),
            // This mode will choose the color with the higher lightness
            MixMode::MinAll => {
                if (self.0.max(self.1).max(self.2)) < (other.0.max(other.1).max(other.2)) {
                    *self
                } else {
                    other
                }
            }
            MixMode::MaxAll => {
                if (self.0.max(self.1).max(self.2)) > (other.0.max(other.1).max(other.2)) {
                    *self
                } else {
                    other
                }
            }
        }
    }

    pub fn gamma(&self) -> f64 {
        self.0.powf(2.2) + self.1.powf(2.2) + self.2.powf(2.2)
    }

    pub fn blank() -> ColorRGBA {
        ColorRGBA(0.0, 0.0, 0.0, 0.0)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        [
            (self.0.clamp(0.0, 1.0) * 255.0) as u8,
            (self.1.clamp(0.0, 1.0) * 255.0) as u8,
            (self.2.clamp(0.0, 1.0) * 255.0) as u8,
            (self.3.clamp(0.0, 1.0) * 255.0) as u8,
        ]
        .to_vec()
    }
}

impl From<ColorHSLA> for ColorRGBA {
    fn from(color: ColorHSLA) -> ColorRGBA {
        let ColorHSLA(h, s, l, a) = color;
        let r: f64;
        let g: f64;
        let b: f64;

        if s == 0.0 {
            r = l;
            g = l;
            b = l;
        } else {
            let hue2rgb = |p: f64, q: f64, mut t: f64| -> f64 {
                while t < 0.0 {
                    t += 1.0;
                }
                while t > 1. {
                    t -= 1.;
                }
                if t < 1. / 6. {
                    p + (q - p) * 6. * t
                } else if t < 1. / 2. {
                    q
                } else if t < 2. / 3. {
                    p + (q - p) * (2. / 3. - t) * 6.
                } else {
                    p
                }
            };

            let q = if l < 0.5 { l * (1. + s) } else { l + s - l * s };
            let p = 2. * l - q;
            r = hue2rgb(p, q, h + 1. / 3.);
            g = hue2rgb(p, q, h);
            b = hue2rgb(p, q, h - 1. / 3.);
        }

        ColorRGBA(r, g, b, a)
    }
}

impl ops::Add for ColorRGBA {
    type Output = ColorRGBA;

    fn add(self, rhs: ColorRGBA) -> Self::Output {
        ColorRGBA::new(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl ops::Sub for ColorRGBA {
    type Output = ColorRGBA;

    fn sub(self, rhs: ColorRGBA) -> Self::Output {
        ColorRGBA::new(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl ops::Mul<f64> for ColorRGBA {
    type Output = ColorRGBA;

    fn mul(self, rhs: f64) -> Self::Output {
        ColorRGBA::new(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl ops::Div<f64> for ColorRGBA {
    type Output = ColorRGBA;

    fn div(self, rhs: f64) -> Self::Output {
        ColorRGBA::new(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}

impl PartialEq for ColorRGBA {
    fn eq(&self, other: &ColorRGBA) -> bool {
        f64_fuzzy_eq(self.0, other.0)
            && f64_fuzzy_eq(self.1, other.1)
            && f64_fuzzy_eq(self.2, other.2)
            && f64_fuzzy_eq(self.3, other.3)
    }
}

impl FuzzyPartialEq<ColorRGBA> for ColorRGBA {
    fn fuzzy_eq(self, other: ColorRGBA) -> bool {
        f64_fuzzy_eq(self.0, other.0)
            && f64_fuzzy_eq(self.1, other.1)
            && f64_fuzzy_eq(self.2, other.2)
            && f64_fuzzy_eq(self.3, other.3)
    }
}

/// A color in HSLA format
///
/// It is composed of four parts:
/// - Hue: The amount of revolutions around the color wheel [-inf, inf]
/// - Saturation: The amount of color [0.0, 1.0]
/// - Lightness: The amount of light, with 0.0 being black, 0.5 being full color, and 1.0 being white [0.0, 1.0]
/// - Alpha: The opacity of the color [0.0, 1.0]
///
/// Example:
/// ```
/// use raytracer::gfx::primitives::{color::{ColorHSLA, ColorRGBA}, mix_modes::MixMode};
///
/// let aqua = ColorHSLA::new(0.5, 1., 0.5, 1.0);
///
/// let aqua_rgb = ColorRGBA::new(0.0, 1.0, 1.0, 1.0);
///
/// assert_eq!(ColorRGBA::from(aqua), aqua_rgb);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ColorHSLA(pub f64, pub f64, pub f64, pub f64);

impl ColorHSLA {
    pub fn new(h: f64, s: f64, l: f64, a: f64) -> ColorHSLA {
        ColorHSLA(h, s, l, a)
    }

    pub fn invert(&self) -> ColorHSLA {
        ColorHSLA(self.0, 1.0 - self.1, 1.0 - self.2, self.3)
    }
}

impl From<ColorRGBA> for ColorHSLA {
    fn from(rgba: ColorRGBA) -> ColorHSLA {
        let r = rgba.0;
        let g = rgba.1;
        let b = rgba.2;
        let a = rgba.3;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);

        let deg60 = 1. / 6.;

        let h = if max == min {
            0.0
        } else if max == r {
            (deg60 * ((g - b) / (max - min)) + deg60 * 0.) % 1.0
        } else if max == g {
            (deg60 * ((b - r) / (max - min)) + deg60 * 2.) % 1.0
        } else if max == b {
            (deg60 * ((r - g) / (max - min)) + deg60 * 4.) % 1.0
        } else {
            0.0
        };

        let s = if max == 0.0 { 0.0 } else { (max - min) / max };

        let l = (max + min) * 0.5;

        ColorHSLA(h, s, l, a)
    }
}

impl PartialEq for ColorHSLA {
    fn eq(&self, other: &ColorHSLA) -> bool {
        f64_fuzzy_eq(self.0, other.0)
            && f64_fuzzy_eq(self.1, other.1)
            && f64_fuzzy_eq(self.2, other.2)
            && f64_fuzzy_eq(self.3, other.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        let c = ColorRGBA::new(1.0, 0.2, 0.4, 1.0);
        assert_eq!(c, ColorRGBA(1.0, 0.2, 0.4, 1.0));
    }

    #[test]
    fn can_add_color_and_color() {
        let c1 = ColorRGBA::new(1.0, 0.2, 0.4, 1.0);
        let c2 = ColorRGBA::new(0.1, 0.1, 0.1, 1.0);
        let c3 = c1 + c2;

        assert_eq!(c3, ColorRGBA(1.1, 0.3, 0.5, 1.0));
    }

    #[test]
    fn can_sub_color_and_color() {
        let c1 = ColorRGBA::new(1.0, 0.2, 0.4, 1.0);
        let c2 = ColorRGBA::new(0.1, 0.1, 0.1, 1.0);
        let c3 = c1 - c2;

        assert_eq!(c3, ColorRGBA(0.9, 0.1, 0.3, 1.0));
    }

    #[test]
    fn can_mul_color_and_f64() {
        let c1 = ColorRGBA::new(1.0, 0.2, 0.4, 1.0);
        let c2 = c1 * 0.5;

        assert_eq!(c2, ColorRGBA(0.5, 0.1, 0.2, 1.0));
    }

    #[test]
    fn can_div_color_and_f64() {
        let c1 = ColorRGBA::new(1.0, 0.2, 0.4, 1.0);
        let c2 = c1 / 0.5;

        assert_eq!(c2, ColorRGBA(2.0, 0.4, 0.8, 1.0));
    }

    #[test]
    fn can_invert_color() {
        let c1 = ColorRGBA::new(1.0, 0.2, 0.4, 1.0);
        let c2 = c1.invert();

        assert_eq!(c2, ColorRGBA(0.0, 0.8, 0.6, 1.0));
    }

    #[test]
    fn can_convert_rgba_to_hsla() {
        let c1 = ColorRGBA::new(1.0, 1.0, 0.0, 1.0);
        let c2 = ColorHSLA::from(c1);

        assert_eq!(c2, ColorHSLA(1. / 6., 1.0, 0.5, 1.0));
    }

    #[test]
    fn can_convert_hsla_to_rgba() {
        let c1 = ColorHSLA::new(1. / 6., 1.0, 0.5, 1.0);
        let c2 = ColorRGBA::from(c1);

        assert_eq!(c2, ColorRGBA(1.0, 1.0, 0.0, 1.0));
    }

    mod mix_modes {
        use super::*;

        #[test]
        fn can_mix_colors_avg() {
            let c1 = ColorRGBA::new(1.0, 0.2, 0.4, 1.0);
            let c2 = ColorRGBA::new(0.1, 0.1, 0.1, 1.0);
            let c3 = c1.mix(c2, MixMode::Avg);

            assert_eq!(c3, ColorRGBA(0.55, 0.15, 0.25, 1.0));
        }

        #[test]
        fn can_mix_colors_alpha() {
            let c1 = ColorRGBA::new(1.0, 0.0, 0.0, 1.0);
            let c2 = ColorRGBA::new(0.0, 1.0, 0.0, 0.5);
            let c3 = c1.mix(c2, MixMode::Alpha);

            assert_eq!(c3, ColorRGBA(0.5, 0.5, 0.0, 1.0));
        }

        #[test]
        fn can_mix_colors_add() {
            let c1 = ColorRGBA::new(1.0, 0.2, 0.4, 1.0);
            let c2 = ColorRGBA::new(0.1, 0.1, 0.1, 1.0);
            let c3 = c1.mix(c2, MixMode::Add);

            assert_eq!(c3, ColorRGBA(1.1, 0.3, 0.5, 1.0));
        }

        #[test]
        fn can_mix_colors_sub() {
            let c1 = ColorRGBA::new(1.0, 0.2, 0.4, 1.0);
            let c2 = ColorRGBA::new(0.1, 0.1, 0.1, 1.0);
            let c3 = c1.mix(c2, MixMode::Sub);

            assert_eq!(c3, ColorRGBA(0.9, 0.1, 0.3, 1.0));
        }

        #[test]
        fn can_mix_colors_mul() {
            let c1 = ColorRGBA::new(1.0, 0.2, 0.4, 1.0);
            let c2 = ColorRGBA::new(0.1, 0.1, 0.1, 1.0);
            let c3 = c1.mix(c2, MixMode::Mul);

            assert_eq!(c3, ColorRGBA(0.1, 0.02, 0.04, 1.0));
        }

        #[test]
        fn can_mix_colors_div() {
            let c1 = ColorRGBA::new(1.0, 0.2, 0.4, 1.0);
            let c2 = ColorRGBA::new(0.1, 0.1, 0.1, 1.0);
            let c3 = c1.mix(c2, MixMode::Div);

            assert_eq!(c3, ColorRGBA(10.0, 2.0, 4.0, 1.0));
        }

        #[test]
        fn can_mix_colors_min() {
            let c1 = ColorRGBA::new(1.0, 0.0, 0.4, 1.0);
            let c2 = ColorRGBA::new(0.1, 0.1, 0.1, 1.0);
            let c3 = c1.mix(c2, MixMode::Min);

            assert_eq!(c3, ColorRGBA(0.1, 0.0, 0.1, 1.0));
        }

        #[test]
        fn can_mix_colors_max() {
            let c1 = ColorRGBA::new(1.0, 0.0, 0.4, 1.0);
            let c2 = ColorRGBA::new(0.1, 0.1, 0.1, 1.0);
            let c3 = c1.mix(c2, MixMode::Max);

            assert_eq!(c3, ColorRGBA(1.0, 0.1, 0.4, 1.0));
        }

        #[test]
        fn can_mix_colors_min_all() {
            let c1 = ColorRGBA::new(1.0, 0.0, 0.4, 1.0);
            let c2 = ColorRGBA::new(0.1, 0.1, 0.1, 1.0);
            let c3 = c1.mix(c2, MixMode::MinAll);

            assert_eq!(c3, ColorRGBA(0.1, 0.1, 0.1, 1.0));
        }

        #[test]
        fn can_mix_colors_max_all() {
            let c1 = ColorRGBA::new(1.0, 0.0, 0.4, 1.0);
            let c2 = ColorRGBA::new(0.1, 0.1, 0.1, 1.0);
            let c3 = c1.mix(c2, MixMode::MaxAll);

            assert_eq!(c3, ColorRGBA(1.0, 0.0, 0.4, 1.0));
        }
    }
}
