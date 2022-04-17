use super::body::Body;

pub struct Intersection {
    pub t: f64,
    pub object: Box<dyn Body>,
}

impl Intersection {
    pub fn new(t: f64, object: Box<dyn Body>) -> Self {
        Self { t, object }
    }
}

pub trait IntersectionList {
    fn hit(&self) -> Option<&Intersection>;
    fn hit_assume_sorted(&self) -> Option<&Intersection>;
}

impl IntersectionList for Vec<Intersection> {
    fn hit_assume_sorted(&self) -> Option<&Intersection> {
        self.iter().find(|i| i.t >= 0.0)
    }

    fn hit(&self) -> Option<&Intersection> {
        // go through Intersections and return the one with smallest t above 0
        self.iter()
            .reduce(|min, i| if i.t >= 0.0 && i.t < min.t { i } else { min })
    }
}
