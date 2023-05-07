//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// error
//

pub fn error_handler(args:&Vec<String>) -> u32
{
    if args.len() != 2 {
        return 1;
    }

    let file: Result<std::fs::File, std::io::Error> = std::fs::File::open(&args[1]);

    match file {
        Ok(_) => return 0,
        Err(_) => return 1,
    }
}