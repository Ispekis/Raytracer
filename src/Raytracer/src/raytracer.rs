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
    light_model:PhongModel
}

impl World {
    pub fn new(scene:FileConfig::SceneData) -> Self {
        let light_model = PhongModel::new(scene.lights.ambient, scene.lights.diffuse, scene.lights.specular);
        Self { scene, light_model }
    }
    pub fn draw_primitives(&mut self, u:f64, v:f64) {
        let ray = self.scene.camera.ray(u, v);

        for i in 0..self.scene.primitives.spheres.len() {
            let hit_point = self.scene.primitives.spheres[i].hits(ray);
            if (hit_point != None) {
                let color = self.light_model.lightning(self.scene.primitives.spheres[i].color, self.scene.lights.point[0],
                    hit_point.unwrap(), ray.direction, self.scene.primitives.spheres[i].suface_normal(hit_point.unwrap()));
                // let normal = self.scene.primitives.spheres[i].suface_normal(hit_point.unwrap());
                // let light_direction = ((hit_point.unwrap() - self.scene.lights.point[0].origin)).normalize();
                // let mut d = normal.scal(&(light_direction * -1.0));
                // if (d < 0.0) {
                //     d = 0.0;
                // }
                // let shadowed = is_shadowed(self.scene, hit_point.unwrap());
                write_flat_color(color);
                // write_color(self.scene.primitives.spheres[i].color, &mut self.scene.lights, d, false);
                return;
            }
        }
        for i in 0..self.scene.primitives.planes.len() {
            let hit_point = self.scene.primitives.planes[i].hits(ray);
            if hit_point != None {
                let color = self.light_model.lightning(self.scene.primitives.planes[i].color, self.scene.lights.point[0],
                    hit_point.unwrap(), ray.direction, self.scene.primitives.planes[i].suface_normal(hit_point.unwrap()));
                write_flat_color(color);
                // write_color(self.scene.primitives.planes[i].color, &mut self.scene.lights, d, shadowed);
                return;
            }
        }
        write_flat_color(Vector3D::new(0.0, 0.0, 0.0));
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

fn intersect(primitives:&FileConfig::Primitives_t, ray:Ray) -> Option<f64> {
    for i in 0..primitives.spheres.len() {
        let hit_point = primitives.spheres[i].hits_get_t(ray);
        if (hit_point != None) {
            return hit_point;
        }
    }
    for i in 0..primitives.planes.len() {
        let hit_point = primitives.planes[i].hits_get_t(ray);
        if hit_point != None{
            return hit_point;
        }
    }
    return None;
}

fn is_shadowed(scene:&mut FileConfig::SceneData, hit_point:Point3D) -> bool {
    let shadow_v = (scene.lights.point[0].origin - hit_point);
    let distance = shadow_v.length();
    let direction = shadow_v.normalize();
    let shadow_ray = Ray::new(hit_point, direction);

    let hit = intersect(&scene.primitives, shadow_ray);

    if (hit != None) {
        if (hit.unwrap() < distance) {
            // println!("{} and {}", hit.unwrap(), distance);
            return true;
        }
    }
    false
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
