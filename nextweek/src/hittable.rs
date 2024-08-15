use crate::aabb;
use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use nalgebra::Vector3;

pub struct HitRecord<'a> {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub material: &'a dyn Material,
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

#[derive(Default)]
pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn push(&mut self, hittable: impl Hittable + 'static) {
        self.list.push(Box::new(hittable))
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything: Option<HitRecord> = None;
        for h in self.list.iter() {
            if let Some(hit) = h.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        hit_anything
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        match self.list.first() {
            Some(first) => {
                match first.bounding_box(t0, t1) {
                    Some(bbox) => self.list.iter().skip(1).try_fold(bbox, |acc, hittable| {
                        hittable.bounding_box(t0, t1).map(|bbox| aabb::surrounding_box(&acc, &bbox))
                    }),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

pub struct FlipNormals<H: Hittable> {
    hittable: H,
}

impl<H: Hittable> FlipNormals<H> {
    pub fn new(hittable: H) -> Self {
        FlipNormals { hittable }
    }
}

impl<H: Hittable> Hittable for FlipNormals<H> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hittable.hit(ray, t_min, t_max).map(|mut hit| {
            hit.normal = -hit.normal;
            hit
        })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.hittable.bounding_box(t0, t1)
    }
}
