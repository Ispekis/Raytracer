//
// EPITECH PROJECT, 2023
// Raytracer
// File description:
// material
//

use crate::math::{
    vector3d::Vector3D,
    point3d::Point3D
};
use crate::ray_tracer::light::PointLight;

pub struct PhongModel {
    ambient:f64,
    diffuse:f64,
}

impl PhongModel {
    pub fn new(ambient:f64, diffuse:f64) -> Self{
        Self {ambient, diffuse}
    }

    pub fn lightning(&self, color:Vector3D, light:PointLight, position:Point3D, normal_v:Vector3D, is_shadow:bool) -> Vector3D {
        let eff_color = color * light.intensity;
        let ambient:Vector3D;
        let diffuse:Vector3D;
        let specular:Vector3D;

        let lightv = (light.origin - position).normalize();

        ambient = eff_color * self.ambient;
        let light_dot_normal = lightv.scal(&normal_v);
        if light_dot_normal < 0.0 {
            diffuse = Vector3D::new(0.0, 0.0, 0.0); // Black
            specular = Vector3D::new(0.0, 0.0, 0.0); // Black
        } else {
            diffuse = eff_color * self.diffuse * light_dot_normal;

            // let reflectv = lightv.reflect(normal_v) * -1.0;
            // let reflect_dot_eye = reflectv.scal(&direction);
            // if (reflect_dot_eye <= 0.0) {
            //     specular = Vector3D::new(0.0, 0.0, 0.0);
            // } else {
            //     let factor = reflect_dot_eye.powf(200.0);
            //     specular = color * self.specular * factor * light.intensity
            // }
            specular = Vector3D::default();
        }
        let mut ret_color: Vector3D;
        if is_shadow {
            ret_color = ambient;
        } else {
            ret_color = ambient + diffuse + specular;
        }
        if ret_color.x >= 255.0 {
            ret_color.x = 255.0;
        }
        if ret_color.y >= 255.0 {
            ret_color.y = 255.0;
        }
        if ret_color.z >= 255.0 {
            ret_color.z = 255.0;
        }
        ret_color
    }
}

impl Default for PhongModel {
    fn default() -> Self {
        Self {
            ambient: 1.0,
            diffuse: 1.0
        }
    }
}
