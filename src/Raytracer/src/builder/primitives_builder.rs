//
// EPITECH PROJECT, 2023
// Raytracer
// File description:
// primitives_builder
//

use crate::math::{
    vector3d::Vector3D,
    point3d::Point3D
};
use crate::canvas::{
    color::Color,
    material,
    material::{
        Mask,
        Solid
    }
};
use crate::interfaces::primitives::Primitives;

pub struct PrimitivesBuilder {
    primitives:Option<Box<dyn Primitives>>,
    center:Option<Point3D>,
    radius:Option<f64>,
    color:Option<Color>,
    pattern:Option<Box<dyn Mask>>,
    reflectiveness:Option<f64>
}


impl PrimitivesBuilder {
    pub fn new() -> Self {
        Self {
            primitives: None,
            center: None,
            radius: None,
            color: None,
            pattern: None,
            reflectiveness: None
        }
    }

    pub fn with_primitives(mut self, primitives:Box<dyn Primitives>) -> Self {
        self.primitives = Some(primitives);
        self
    }

    pub fn with_center(mut self, center:Point3D) -> Self {
        self.center = Some(center);
        self
    }

    pub fn with_radius(mut self, radius:f64) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn with_color(mut self, color:Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_pattern(mut self, pattern:Box<dyn Mask>) -> Self {
        self.pattern = Some(pattern);
        self
    }

    pub fn with_reflectiveness(mut self, reflectiveness:f64) -> Self {
        self.reflectiveness = Some(reflectiveness);
        self
    }

    pub fn build(self) -> std::result::Result<Box<dyn Primitives>, Box<dyn std::error::Error>> {
        // let center = self.center;
        // let radius = self.radius;
        // let color = self.color;
        // // set material
        // let mut pattern: Box<dyn material::Mask>;
        // if self.pattern.is_none() {
        //     pattern = Box::new(Solid::new(color));
        // } else {
        //     pattern = self.pattern.unwrap();
        // }

        // // set reflectiveness
        // let reflectiveness: f64;
        // if self.reflectiveness.is_none() {
        //     reflectiveness = 0.0;
        // } else {
        //     reflectiveness = self.reflectiveness.unwrap();
        // }

        let mut primitives = self.primitives.ok_or("primitives not specified")?;

        primitives.with_center(self.center)?;
        primitives.with_color(self.color)?;
        primitives.with_pattern(self.pattern)?;
        primitives.with_radius(self.radius)?;
        primitives.with_reflectiveness(self.reflectiveness)?;

        Ok(primitives)
    }
}
