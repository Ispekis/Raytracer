//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// raytracer
//

use crate::config::fileconfig;
use crate::interfaces::primitives::Primitives;
use crate::math::{point3d::Point3D, vector3d::Vector3D};
use crate::ray_tracer::{
    ray::Ray,
    material::PhongModel
};

struct World {
    scene:fileconfig::SceneData,
    objects:Vec<Box<dyn Primitives>>,
    light_model:PhongModel,
    reflection_limit:usize
}

impl World {
    pub fn new(scene:fileconfig::SceneData) -> Self {
        let light_model = PhongModel::new(scene.lights.ambient, scene.lights.diffuse, scene.lights.specular);
        let mut objects: Vec<Box<dyn Primitives>> = Vec::new();

        for i in 0..scene.primitives.spheres.len() {
            objects.push(Box::new(scene.primitives.spheres[i].clone()))
        }
        for i in 0..scene.primitives.planes.len() {
            objects.push(Box::new(scene.primitives.planes[i].clone()))
        }
        for i in 0..scene.primitives.cylinders.len() {
            objects.push(Box::new(scene.primitives.cylinders[i].clone()))
        }
        for i in 0..scene.primitives.cones.len() {
            objects.push(Box::new(scene.primitives.cones[i].clone()))
        }
        Self { scene, objects, light_model, reflection_limit:5 }
    }

    pub fn color_at(&self, ray:Ray) -> Vector3D {
        self.color_at_with_reflection(ray, self.reflection_limit)
    }

    pub fn color_at_with_reflection(&self, ray:Ray, remain_reflection:usize) -> Vector3D {
        let mut color:Vector3D = Vector3D::new(0.0, 0.0, 0.0);
        let mut distance:f64 = f64::INFINITY;

        for i in 0..self.objects.len() {
            let hit_res = self.objects[i].hits(ray);
            if let Some(hit_point) = hit_res {
                if (hit_point - ray.origin).length() < distance {
                    let reflectv = ray.direction.reflect(self.objects[i].suface_normal(hit_point));
                    color = self.light_model.lightning(self.objects[i].get_pattern().color_at(hit_point), self.scene.lights.point[0],
                    hit_point, self.objects[i].suface_normal(hit_point),
                    self.is_shadowed(hit_point, i));
                    distance = (hit_point - ray.origin).length();

                    let reflected_color = self.reflected_color_at(hit_point, reflectv, remain_reflection, i);

                    color = color + reflected_color;
                }
            }
        }
        color
    }

    pub fn draw_primitives(&mut self, u:f64, v:f64) {
        let ray = self.scene.camera.ray(u, v);
        // let mut color:Vector3D = Vector3D::new(0.0, 0.0, 0.0);
        // let mut distance:f64 = f64::INFINITY;

        // for i in 0..self.objects.len() {
        //     let hit_res = self.objects[i].hits(ray);
        //     if let Some(hit_point) = hit_res {
        //         if (hit_point - ray.origin).length() < distance {
        //             let reflectv = ray.direction.reflect(self.objects[i].suface_normal(hit_point));
        //             color = self.light_model.lightning(self.objects[i].get_pattern().color_at(hit_point), self.scene.lights.point[0],
        //             hit_point, self.objects[i].suface_normal(hit_point),
        //             self.is_shadowed(hit_point, i));
        //             distance = (hit_point - ray.origin).length();

        //             let reflected_color = self.reflected_color_at(hit_point, reflectv, i);

        //             color = color + reflected_color;
        //         }
        //     }
        // }
        let color = self.color_at(ray);
        write_flat_color(color);
        return;
    }

    fn intersect(&self, ray:Ray, object_index:usize) -> Option<Point3D> {
        for i in 0..self.objects.len() {
            if i == object_index {
                continue;
            }
            let hit_point = self.objects[i].hits(ray);
            if hit_point != None {
                return hit_point;
            }
        }
        return None;
    }

    fn reflected_color_at(&self, hit_point:Point3D, reflectv:Vector3D, remain_reflection:usize, index:usize) -> Vector3D{
        // Hit non reflective object
        if self.objects[index].get_reflectiveness() == 0.0 || remain_reflection == 0 {
            return Vector3D::new(0.0, 0.0, 0.0);
        }

        let reflected_ray = Ray::new(hit_point, reflectv);

        let reflected_color:Vector3D = self.color_at_with_reflection(reflected_ray, remain_reflection - 1);
        reflected_color * self.objects[index].get_reflectiveness()
    }

    fn is_shadowed(&self, hit_point:Point3D, object_index:usize) -> bool {
        let shadow_v = self.scene.lights.point[0].origin - hit_point;
        let distance = shadow_v.length();
        let direction = shadow_v.normalize();
        let shadow_ray = Ray::new(hit_point, direction);

        let hit = self.intersect(shadow_ray, object_index);

        if let Some(t) = hit {
            if (t - shadow_ray.origin).length() < distance {
                return true;
            }
        }
        false
    }
}

fn write_flat_color(color:Vector3D) {
    println!("{} {} {}", color.x as u32, color.y as u32, color.z as u32);
}

pub fn run_raytracer(scene:fileconfig::SceneData) -> u32
{
    let width = scene.camera.width;
    let height = scene.camera.height;

    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    let mut world = World::new(scene);

    for y in 0..height {
        for x in 0..width {
            let u = x as f64 / (width as f64 - 1.0);
            let v = y as f64 / (height as f64 - 1.0);
            world.draw_primitives(u, v);
        }
    }
    return 0;
}
