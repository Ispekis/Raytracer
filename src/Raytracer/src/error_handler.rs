//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// error
//

pub fn error_handler(args:&Vec<String>) -> (u32, usize)
{
    let mut file_index: usize = 1;

    if args.len() != 2 && !(args.len() == 3 && (&args[1] == "--bonus" || &args[2] == "--bonus")) {
        return (1, 0);
    }
    if args.len() == 3 && &args[1] == "--bonus" {
        file_index = 2;
    }

    let file: Result<std::fs::File, std::io::Error> = std::fs::File::open(&args[file_index]);

    match file {
        Ok(_) => {
            if args.len() == 3 && (&args[1] == "--bonus" || &args[2] == "--bonus") {
                return (2, file_index);
            }
            return (0, file_index);
        }
        Err(_) => return (1, 0),
    }
}