use crate::aabb::AABB;
use crate::hittable::{FlipNormals, HitRecord, Hittable, HittableList};
use crate::material::Material;
use crate::ray::Ray;
use crate::rect::{AARect, Plane};
use nalgebra::Vector3;

pub struct Cube {
    p_min: Vector3<f32>,
    p_max: Vector3<f32>,
    sides: HittableList,
}

impl Cube {
    pub fn new<M: Material + Clone + 'static>(
        p_min: Vector3<f32>,
        p_max: Vector3<f32>,
        material: M,
    ) -> Self {
        let mut sides = HittableList::default();
        sides.push(AARect::new(
            Plane::XY,
            p_min.x,
            p_max.x,
            p_min.y,
            p_max.y,
            p_max.z,
            material.clone(),
        ));
        sides.push(FlipNormals::new(AARect::new(
            Plane::XY,
            p_min.x,
            p_max.x,
            p_min.y,
            p_max.y,
            p_min.z,
            material.clone(),
        )));
        sides.push(AARect::new(
            Plane::ZX,
            p_min.z,
            p_max.z,
            p_min.x,
            p_max.x,
            p_max.y,
            material.clone(),
        ));
        sides.push(FlipNormals::new(AARect::new(
            Plane::ZX,
            p_min.z,
            p_max.z,
            p_min.x,
            p_max.x,
            p_min.y,
            material.clone(),
        )));
        sides.push(AARect::new(
            Plane::YZ,
            p_min.y,
            p_max.y,
            p_min.z,
            p_max.z,
            p_max.x,
            material.clone(),
        ));
        sides.push(FlipNormals::new(AARect::new(
            Plane::YZ,
            p_min.y,
            p_max.y,
            p_min.z,
            p_max.z,
            p_min.x,
            material,
        )));
        Cube {
            p_min,
            p_max,
            sides,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.p_min,
            max: self.p_max,
        })
    }
}
