mod error_handler;
mod usage;

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 && &args[1] == "--help" {
        usage::display_usage(&args[0]);
        return std::process::ExitCode::SUCCESS;
    }
    if error_handler::error_handler(&args) == 1 {
        return std::process::ExitCode::from(84);
    }
    return std::process::ExitCode::SUCCESS;
}
