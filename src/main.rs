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

mod Config {
    pub mod FileConfig;
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

    let scene = Config::FileConfig::SceneData::new(&args[1]);

    match scene {
        Ok((s)) => {
            raytracer::run_raytracer(s);
            return std::process::ExitCode::SUCCESS;
        },
        Err((_)) => return std::process::ExitCode::from(84)
    }
}