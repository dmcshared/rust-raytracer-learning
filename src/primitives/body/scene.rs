// WIP
// use crate::primitives::{intersection::Intersection, ray::Ray};

// use super::Body;

// #[derive(Debug)]
// pub struct Scene {
//     pub bodies: Vec<Arc<dyn Body>>,
// }

// impl Clone for Scene {
//     fn clone(&self) -> Self {
//         Self {
//             bodies: self
//                 .bodies
//                 .iter()
//                 .map(|b| dyn_clone::clone_box(&**b))
//                 .collect(),
//         }
//     }
// }

// impl<'a> Body for Scene {
//     fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
//         self.bodies
//             .iter()
//             .flat_map(|b| b.intersect(ray))
//             .map(|b| b.with_top_level_object(self))
//             .collect()
//     }

//     fn normal_raw(&self, x: f64, y: f64, z: f64) -> crate::primitives::three_part::vector::Vector {
//         todo!()
//     }

//     fn get_material(&self) -> Arc<dyn crate::primitives::material::Material> {
//         todo!()
//     }
// }
