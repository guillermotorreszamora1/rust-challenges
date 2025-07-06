use std::{env, fs};
use std::os::unix::fs::PermissionsExt;

fn main() {
    let args: Vec<String> = env::args().collect();

    let programs_to_print;
    dbg!(&args);
    let print_all;
    if args.len() < 2 {
        return;
    }
    if args.get(1).unwrap() == "-a" {
        print_all = true;
        programs_to_print = &args[2..];
    } else {
        print_all = false;
        programs_to_print = &args[1..];
    }

    let path: String;
    if let Ok(path_ok) = env::var("PATH") {
        path = path_ok;
    } else{
        return;
    }

    for program in programs_to_print {
        for dir in path.split(":") {
            let full_name = format!("{}/{}",dir,program);
            let metadata_result = fs::metadata(&full_name);
            if let Ok(metadata) = metadata_result {
                let permissions_mode = metadata.permissions().mode();
                if (permissions_mode & 0o100)  != 0 {
                    println!("{full_name}");
                    if !print_all {
                        break;
                    }
                }
            }
        }
    }
}
