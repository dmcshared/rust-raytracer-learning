#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub f64, pub f64, pub f64);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        let c = Color::new(1.0, 0.2, 0.4);
        assert_eq!(c, Color(1.0, 0.2, 0.4));
    }
}
