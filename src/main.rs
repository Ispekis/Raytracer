use libc::POLLRDBAND;
use solid::plane;

mod error_handler;
mod usage;
mod FileConfig;
mod solid {
    pub mod plane;
    pub mod sphere;
    pub mod rgb;
    pub mod Point3D;
    pub mod Vector3D;
    pub mod Isolid;
}

// mod Point3D;


fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 && &args[1] == "--help" {
        usage::display_usage(&args[0]);
        return std::process::ExitCode::SUCCESS;
    }
    if error_handler::error_handler(&args) == 1 {
        return std::process::ExitCode::from(84);
    }
    // FileConfig::write_config();
    let t = plane::Plane::new_default();
    let a = solid::rgb::Rgb::default();
    // let test = Plane::Default;
    return std::process::ExitCode::SUCCESS;
}