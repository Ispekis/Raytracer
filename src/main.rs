mod error_handler;
mod usage;
mod raytracer;
mod Math {
    pub mod Point3D;
    pub mod Vector3D;
    pub mod formulas;
}

pub mod ISolid;
pub mod Rgb;

mod tools;

mod RayTracer {
    pub mod Ray;
    pub mod Sphere;
    pub mod Camera;
    pub mod Plane;
    pub mod Rectangle3D;
    pub mod Light;
}

mod Parsing {
    pub mod FileConfig;
}

fn write_color(color:Math::Vector3D::Vector3D) {
    println!("{} {} {}", color.x as u32, color.y as u32, color.z as u32);
}

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 && &args[1] == "--help" {
        usage::display_usage(&args[0]);
        return std::process::ExitCode::SUCCESS;
    }
    if error_handler::error_handler(&args) == 1 {
        return std::process::ExitCode::from(84);
    }

    let scene = Parsing::FileConfig::SceneData::new(&args[1]);

    // match scene {
    //     Ok((v)) => return std::process::ExitCode::SUCCESS,
    //     Err((_)) => return std::process::ExitCode::from(84)
    // }

    let cam = RayTracer::Camera::Camera::default();
    let s = RayTracer::Sphere::Sphere::new(Math::Point3D::Point3D::new(0.0, 0.0, -1.0), 0.5);

    let width = 100;
    let height = 50;

    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    for y in 0..height {
        for x in 0..width {
            let u = x as f64 / (width as f64 - 1.0);
            let v = y as f64 / (height as f64 - 1.0);
            let ray = cam.ray(u, v);
            // println!("ray {} {} {}", ray.direction.x, ray.direction.y, ray.direction.z);
            if s.hits(ray) {
                // println!("hit at u {} and v {}", u, v);
                // write_color(Math::Vector3D::Vector3D::new(255.0, 0.0, 0.0));
            } else {
                // write_color(Math::Vector3D::Vector3D::new(0.0, 0.0, 255.0));
                // println!("no");
            }
        }
    }

    return std::process::ExitCode::SUCCESS;
}