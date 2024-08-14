use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use nalgebra::Vector3;

pub struct Translate<H: Hittable> {
    hittable: H,
    offset: Vector3<f32>,
}

impl<H: Hittable> Translate<H> {
    pub fn new(hittable: H, offset: Vector3<f32>) -> Self {
        Translate { hittable, offset }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());
        self.hittable.hit(&moved_ray, t_min, t_max).map(|mut hit| {
            hit.p += self.offset;
            hit
        })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.hittable.bounding_box(t0, t1).map(|mut b| {
            b.min += self.offset;
            b.max += self.offset;
            b
        })
    }
}
