mod point;
mod vector;

mod three_part {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct ThreePart {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub w: f64,
    }
}
