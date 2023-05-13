mod error_handler;
mod usage;
mod raytracer;
mod math {
    pub mod point3d;
    pub mod vector3d;
    pub mod formulas;
}

mod tools;

mod canvas {
    pub mod color;
    pub mod sphere;
    pub mod plane;
    pub mod cylinder;
    pub mod cone;
    pub mod material;
}

mod ray_tracer {
    pub mod ray;
    pub mod camera;
    pub mod rectangle3d;
    pub mod light;
}

mod interfaces {
    pub mod primitives;
}


mod config {
    pub mod fileconfig;
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

    let scene = config::fileconfig::SceneData::new(&args[1]);

    match scene {
        Ok(s) => {
            // let mutable_scene = &mut s;
            raytracer::run_raytracer(s);
            return std::process::ExitCode::SUCCESS;
        },
        Err(_) => {
            eprintln!("Error in reading the scene config");
            return std::process::ExitCode::from(84)
        }
    }
}