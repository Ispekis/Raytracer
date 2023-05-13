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
    specular:f64,
}

impl PhongModel {
    pub fn new(ambient:f64, diffuse:f64, specular:f64) -> Self {
        Self {ambient, diffuse, specular}
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

            self.specular; // avoid warnings
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
            diffuse: 1.0,
            specular: 1.0
        }
    }
}

pub trait Mask {
    fn color_at(&self, position:Point3D) -> Vector3D;
    fn box_clone(&self) -> Box<dyn Mask>;
    fn set_color(&mut self, color:Vector3D);
}

impl Clone for Box<dyn Mask> {
    fn clone(&self) -> Box<dyn Mask> {
        self.box_clone()
    }
}

// pub enum Pattern {
//     Solid(Solid),
//     Chessboard(Chessboard)
// }

#[derive(Copy, Clone)]
pub struct Solid {
    color: Vector3D
}

impl Solid {
    pub fn new(color:Vector3D) -> Self {
        Self { color }
    }
}

pub fn get_material_pattern(str: &str) -> Box<dyn Mask> {
    if str == "\"solid\"" {
        return Box::new(Solid::default());
    }
    if str == "\"chessboard\"" {
        return Box::new(Chessboard::default())
    }
    Box::new(Solid::default())
}

impl Default for Solid {
    fn default() -> Self {
        Solid { color: Vector3D::default() }
    }
}

impl Mask for Solid {
    fn color_at(&self, _position:Point3D) -> Vector3D {
        self.color
    }

    fn box_clone(&self) -> Box<dyn Mask> {
        Box::new(*self)
    }

    fn set_color(&mut self, color:Vector3D) {
        self.color = color;
    }
}

#[derive(Copy, Clone)]
pub struct Chessboard {
    color_a: Vector3D,
    color_b: Vector3D
}

impl Default for Chessboard {
    fn default() -> Self {
        Self { color_a: Vector3D::new(0.0, 0.0, 0.0), color_b: Vector3D::new(255.0, 255.0, 255.0) }
    }
}

impl Mask for Chessboard {
    fn color_at(&self, position:Point3D) -> Vector3D {
        let x = position.x;
        let y = position.y;
        let z = position.z;

        if (x.floor() + y.floor() + z.floor()) as i64 % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }

    fn box_clone(&self) -> Box<dyn Mask> {
        Box::new(*self)
    }

    fn set_color(&mut self, _color:Vector3D) {}
}

pub trait Mask {
    fn color_at(&self, position:Point3D) -> Vector3D;
    fn box_clone(&self) -> Box<dyn Mask>;
    fn set_color(&mut self, color:Vector3D);
}

impl Clone for Box<dyn Mask> {
    fn clone(&self) -> Box<dyn Mask> {
        self.box_clone()
    }
}

// pub enum Pattern {
//     Solid(Solid),
//     Chessboard(Chessboard)
// }

#[derive(Copy, Clone)]
pub struct Solid {
    color: Vector3D
}

impl Solid {
    pub fn new(color:Vector3D) -> Self {
        Self { color }
    }
}

pub fn get_material_pattern(str: &str) -> Box<dyn Mask> {
    if str == "\"solid\"" {
        return Box::new(Solid::default());
    }
    if str == "\"chessboard\"" {
        return Box::new(Chessboard::default())
    }
    Box::new(Solid::default())
}

impl Default for Solid {
    fn default() -> Self {
        Solid { color: Vector3D::default() }
    }
}

impl Mask for Solid {
    fn color_at(&self, _position:Point3D) -> Vector3D {
        self.color
    }

    fn box_clone(&self) -> Box<dyn Mask> {
        Box::new(*self)
    }

    fn set_color(&mut self, color:Vector3D) {
        self.color = color;
    }
}

#[derive(Copy, Clone)]
pub struct Chessboard {
    color_a: Vector3D,
    color_b: Vector3D
}

impl Default for Chessboard {
    fn default() -> Self {
        Self { color_a: Vector3D::new(0.0, 0.0, 0.0), color_b: Vector3D::new(255.0, 255.0, 255.0) }
    }
}

impl Mask for Chessboard {
    fn color_at(&self, position:Point3D) -> Vector3D {
        let x = position.x;
        let y = position.y;
        let z = position.z;

        if (x.floor() + y.floor() + z.floor()) as i64 % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }

    fn box_clone(&self) -> Box<dyn Mask> {
        Box::new(*self)
    }

    fn set_color(&mut self, _color:Vector3D) {}
}
