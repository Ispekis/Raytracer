//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// triangle
//

use crate::math::{vector3d::Vector3D, point3d::Point3D};
use crate::interfaces::{Primitives, Mask};
use crate::matrix::Transformation;
use crate::ray_tracer::ray::Ray;
use crate::canvas::{
    color::Color,
    material::{
        Solid
    }
};

#[derive(Clone)]
pub struct Triangle {
    pub center:Point3D,
    pub radius:f64,
    pub height:f64,
    pub color:Color,
    pub axis:char,
    pub direction:Vector3D,
    pub pattern:Box<dyn Mask>,
    pub reflectiveness:f64,
    transformation: Transformation,
    pub a:Vector3D,
    pub b:Vector3D,
    pub c:Vector3D
}

impl Primitives for Triangle {
    fn hits(&self, ray:Ray) -> Option<Point3D> {
        return None;
    }
    fn translate(&mut self, translate:Vector3D) {
        self.center.x += &translate.x;
        self.center.y += &translate.y;
        self.center.z += &translate.z;
    }
    fn rotatex(&mut self, _:f64) {}
    fn rotatey(&mut self, _:f64) {}
    fn rotatez(&mut self, _:f64) {}
    fn scale(&mut self, value:f64) {
        self.radius *= value;
    }
    fn suface_normal(&self, _:Point3D) -> Vector3D {
        return Vector3D::default();
    }
    fn get_color(&self) -> Color {
        self.color
    }
    fn get_pattern(&self) -> Box<dyn Mask> {
        self.pattern.clone()
    }
    fn get_reflectiveness(&self) -> f64 {
        self.reflectiveness
    }

    fn clone_box(&self) -> Box<dyn Primitives> {
        Box::new(Self {
            center: self.center,
            radius: self.radius,
            height: self.height,
            color: self.color,
            axis: self.axis,
            direction: self.direction,
            pattern: self.pattern.clone(),
            reflectiveness: self.reflectiveness,
            transformation: self.transformation
        })
    }

    fn with_center(&mut self, center:Option<Point3D>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if center.is_none() {
            return Err("Missing center".into());
        }
        if self.axis == 'X' {
            self.direction.x = 1.0;
        }
        if self.axis == 'Y' {
            self.direction.y = 1.0;
        }
        if self.axis == 'Z' {
            self.direction.y = 1.0;
        }
        self.center = center.unwrap();
        Ok(())
    }

    fn with_radius(&mut self, radius:Option<f64>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if radius.is_none() {
            return Err("Missing radius".into());
        }
        self.radius = radius.unwrap();
        Ok(())
    }

    fn with_color(&mut self, color:Option<Color>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if color.is_none() {
            return Err("Missing color".into());
        }
        self.color = color.unwrap();
        Ok(())
    }

    fn with_pattern(&mut self, pattern:Option<Box<dyn Mask>>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if pattern.is_none() {
            self.pattern = Box::new(Solid::new(self.color));
        } else {
            self.pattern = pattern.unwrap();
        }
        Ok(())
    }

    fn with_reflectiveness(&mut self, reflectiveness:Option<f64>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if reflectiveness.is_none() {
            self.reflectiveness = 0.0;
        } else {
            self.reflectiveness = reflectiveness.unwrap();
        }
        Ok(())
    }

    fn with_axis(&mut self, axis:Option<char>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if axis.is_none() {
            self.axis = 'Z';
        } else {
            self.axis = axis.unwrap();
        }
        Ok(())
    }

    fn with_height(&mut self, height:Option<f64>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if height.is_none() {
            self.height = 0.0;
        } else {
            self.height = height.unwrap();
        }
        Ok(())
    }

    fn with_scale(&mut self, scale:Option<f64>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if scale.is_none() {
            self.transformation.scale = 1.0;
        } else {
            self.transformation.scale = scale.unwrap();
        }
        Ok(())
    }

    fn with_translation(&mut self, translation:Option<Vector3D>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if translation.is_none() {
            self.transformation.translation = Vector3D::default();
        } else {
            self.transformation.translation = translation.unwrap();
        }
        Ok(())
    }

    fn with_rotation(&mut self, rotation:Option<Vector3D>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if rotation.is_none() {
            self.transformation.rotation = Vector3D::default();
        } else {
            self.transformation.rotation = rotation.unwrap();
        }
        Ok(())
    }
}