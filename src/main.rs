//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// main
//

mod error_handler;
mod usage;
mod raytracer;
mod Vector3D;

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 && &args[1] == "--help" {
        usage::display_usage(&args[0]);
        return std::process::ExitCode::SUCCESS;
    }
    if error_handler::error_handler(&args) == 1 {
        return std::process::ExitCode::from(84);
    }
    raytracer::run_raytracer();

    let vec1 = Vector3D::Vector3D::new_value(2.0, 4.0, 0.0);
    let vec2 = Vector3D::Vector3D::new_value(2.0, 4.0, 0.0);

    // println!("{}", Vector3D::length(&vec1));
    println!("{}", (vec1 / vec2).z);
    return std::process::ExitCode::SUCCESS;
}
