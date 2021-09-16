mod math;
mod objects;

use std::ops::*;
use std::rc::Rc; // Use this to allow multiple geometry to share the same instance.

pub use crate::math::degrees_to_radians;

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
}

pub struct HittableList {
    pub object_list: Vec<Rc<dyn Hittable>>, // Using a Box as we don't want to be copying around many objects.
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }

    pub fn length_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_sq().sqrt()
    }

    pub fn add(&self, f: f64) -> Vec3 {
        Vec3 {
            x: self.x + f,
            y: self.y + f,
            z: self.z + f,
        }
    }

    pub fn sub(&self, f: f64) -> Vec3 {
        self.add(f * -1.0)
    }

    pub fn div(&self, f: f64) -> Vec3 {
        self.mul(1.0/f)
    }

    pub fn mul(&self, f: f64) -> Vec3 {
        Vec3 {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }

    pub fn cross(&self, o: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x,
        }
    }

    pub fn dot(&self, o: &Vec3) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn unit_vector(&self) -> Vec3 {
        self.div(self.length())
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, f:Self) -> Self {
        Self {
            x: self.x + f.x,
            y: self.y + f.y,
            z: self.z + f.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, f:Self) -> Self {
        Self {
            x: self.x - f.x,
            y: self.y - f.y,
            z: self.z - f.z,
        }
    }
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn at(&self, mag: f64) -> Vec3 {
        self.origin.clone() + self.direction.mul(mag)
    }
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f64) -> Sphere {
        Sphere {
            centre,
            radius
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        let front_face = ray.direction.dot(outward_normal) < 0.0;

        self.normal = if front_face {
            outward_normal.clone()
        } else {
            outward_normal.mul(-1.0)
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Calculate the discriminant
        let oc = ray.origin.clone() - self.centre.clone();
        let a = ray.direction.length_sq();
        let half_b = oc.dot(&ray.direction); // we removed the '2' as we can consider the case b = 2h
        let c = oc.length_sq() - (self.radius * self.radius);
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false
        }
        let sqrt_d = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        rec.normal = (rec.point.clone() - self.centre.clone()).div(self.radius);
        let outward_normal = (rec.point.clone() - self.centre.clone()).div(self.radius);
        rec.set_face_normal(ray, &outward_normal);

        return true;
    }
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            object_list: vec![],
        }
    }

    pub fn clear(&mut self) -> () {
        self.object_list.clear()
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) -> () {
        self.object_list.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {

        let mut temp_record = HitRecord {
            point: Default::default(),
            normal: Default::default(),
            t: 0.0,
            front_face: false
        };

        let mut hit_anything = false;
        let mut closest_current = t_max;

        // Iterate over each object in the list of Hittable.
        for i in &self.object_list {
            if i.hit(ray, t_min, closest_current, &mut temp_record) {
                hit_anything = true;
                closest_current = temp_record.clone().t;

                // Find a better way to do this- basically changing rec to the values in temp_record.
                rec.point = temp_record.point.clone();
                rec.normal = temp_record.normal.clone();
                rec.t = temp_record.t.clone();
                rec.front_face = temp_record.front_face.clone();
            }
        }
        hit_anything
    }
}

impl Default for Vec3 {
    // Returns a zero vector.
    fn default() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }
}

impl Default for Ray {
    fn default() -> Ray {
        Ray {
            origin: Default::default(),
            direction: Default::default(),
        }
    }
}

impl Default for HitRecord {
    fn default() -> HitRecord {
        HitRecord {
            point: Default::default(),
            normal: Default::default(),
            t: 0.0,
            front_face: false
        }
    }
}

impl Default for HittableList {
    fn default() -> HittableList {
        HittableList {
            object_list: vec![]
        }
    }
}

impl Clone for HitRecord {
    fn clone(&self) -> HitRecord {
        HitRecord {
            point: self.point.clone(),
            normal: self.normal.clone(),
            t: self.t,
            front_face: self.front_face
        }
    }
}

pub fn unit_vector(t: f64) -> Vec3 {
    Vec3 {
        x: t,
        y: t,
        z: t,
    }
}

pub fn find_unit_vector(v: &Vec3) -> Vec3 {
    let mag = v.length();
    v.div(mag)
}