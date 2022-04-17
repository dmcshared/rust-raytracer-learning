use super::{
    body::Body,
    ray::Ray,
    three_part::{point::Point, vector::Vector},
};

pub struct Intersection {
    pub t: f64,
    pub object: Box<dyn Body>,
    pub top_level_object: Box<dyn Body>,
    pub ray: Ray,
    pub world_pos: Point,
    pub world_normal: Vector,
}

impl Intersection {
    pub fn new(t: f64, object: Box<dyn Body>, ray: Ray) -> Self {
        Self {
            t,
            world_pos: ray.at(t),
            world_normal: object.normal(ray.at(t)),
            top_level_object: dyn_clone::clone_box(&*object),
            object,
            ray,
        }
    }

    pub fn with_top_level_object(mut self, object: Box<dyn Body>) -> Self {
        self.top_level_object = object;
        self
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
