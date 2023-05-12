//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// raytracer
//

use crate::Config::FileConfig;
use crate::Interfaces::Primitives::Primitives;
use crate::Math::{Point3D::Point3D, Vector3D::Vector3D, formulas};
use crate::RayTracer::{
    Light::Light,
    Ray::Ray,
    material::PhongModel
};

struct World {
    scene:FileConfig::SceneData,
    objects:Vec<Box<dyn Primitives>>,
    light_model:PhongModel
}

impl World {
    pub fn new(scene:FileConfig::SceneData) -> Self {
        let light_model = PhongModel::new(scene.lights.ambient, scene.lights.diffuse, scene.lights.specular);
        let mut objects: Vec<Box<dyn Primitives>> = Vec::new();

        for i in 0..scene.primitives.spheres.len() {
            objects.push(Box::new(scene.primitives.spheres[i]))
        }
        for i in 0..scene.primitives.planes.len() {
            objects.push(Box::new(scene.primitives.planes[i]))
        }
        Self { scene, objects, light_model }
    }

    pub fn draw_primitives(&mut self, u:f64, v:f64) {
        let ray = self.scene.camera.ray(u, v);

        for i in 0..self.objects.len() {
            let hit_res = self.objects[i].hits(ray);
            if let Some(hit_point) = hit_res {
                let color = self.light_model.lightning(self.objects[i].get_color(), self.scene.lights.point[0],
                hit_point, ray.direction, self.objects[i].suface_normal(hit_point),
                self.is_shadowed(hit_point, i));

                write_flat_color(color);
                return;
            }
        }
        write_flat_color(Vector3D::new(0.0, 0.0, 0.0));
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

fn write_color(color:Vector3D, light:&mut Light, coeff:f64, is_shadow:bool) {
    let color_light = light.point[0].color * light.diffuse;
    let mut r: f64;
    let mut g: f64;
    let mut b: f64;
    if is_shadow {
        r = color.x * light.ambient;
        g = color.y * light.ambient;
        b = color.z * light.ambient;
    } else {
        r = color.x * light.ambient + (color_light.x * coeff);
        g = color.y * light.ambient + (color_light.y * coeff);
        b = color.z * light.ambient + (color_light.z * coeff);
    }

    write_flat_color(Vector3D::new(r, g, b));
}

pub fn run_raytracer(scene:FileConfig::SceneData) -> u32
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
