use std::fs;
use text_colorizer::*;
use std::env;

#[derive(Debug)]
struct Arguments { 
	size: u64,
//	measure: char, //k, m, g, t (kbytes, megabytes, gigabytes, terabytes)
	dir: String    // must be a directory, not a regular file or link.
}

fn main() {
    let args = process_arguments();
    process_directories(&args);
    println!("{:?}", &args); 
}

fn process_directories(args: &Arguments) -> u64{ 
    let mut total_size: u64 = 0;
    let dir_items = match fs::read_dir(&args.dir) {
        Ok(x) => x, 
        Err(_)        => {  // We don't need to fail here. Just skip.
            eprintln!("{}: Unable to read {}", "Invalid entry".red(), args.dir);
            return 0; 
        }
    };
    
    for f in dir_items {
        let entry = match f {
            Ok(x) => x, 
            Err(_)          => { // We don't need to fail here. Just skip.
                eprintln!("{}: Unable to process {}", "Invalid entry".red(), args.dir);
                continue; 
            }
        };
        let meta = match entry.metadata() {
            Ok(x)  => x,
            Err(_)           => { 
                eprintln!("{}: Unable to read metadata for {}", "Invalid entry".red(), entry.path().display());
                continue; 
            } 
        };

        if meta.is_dir() {
            let new_args = Arguments { size: args.size, dir: entry.path().display().to_string() };
            let dir_size = process_directories(&new_args);
            if dir_size >= args.size {
                println!("{} bytes: {} ", format_size(dir_size), entry.path().display());
            }
            total_size += dir_size;
            // proceed recursively. Then print the size.
        } else if meta.is_file() { 
            let file_size = meta.len();
            if file_size >= args.size {
                println!("{} bytes: {}", format_size(file_size), entry.path().display());
            }
            total_size += file_size;
        } else if meta.is_symlink() {
            // For simplicity, we should probably ignore symbolic links. They create the complication that 
            // they may point to folders that are completely outside of the selected hierarchy. They may
            // even create loops!
//            println!("Found a symlink: {}", entry.path().display());
        }   
    }
    return total_size; 
}

fn format_size(size_in_bytes: u64) -> String {
    if size_in_bytes < 1024 {
        return format!("{} bytes", size_in_bytes);
    } else if size_in_bytes < 1024 * 1024 {
        return format!("{} KB", size_in_bytes / 1024);
    } else if size_in_bytes < 1024 * 1024 * 1024 {
        return format!("{} MB", size_in_bytes / (1024 * 1024));
    } else if size_in_bytes < 1024 * 1024 * 1024 * 1024 {
        return format!("{} GB", size_in_bytes / (1024 * 1024 * 1024));
    } else {
        return format!("{} TB", size_in_bytes / (1024 * 1024 * 1024 * 1024));
    }
}
    

fn process_arguments() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut size = 1;
    let mut measure = 'm';
    let mut dir = ".".to_string();

    if args.len() == 0 {
        print_usage();
        std::process::exit(1);
    } else {
        println!("{:?}", args);
    }

    if args.len() >= 1 {
        if args[0].starts_with("-") {
            measure = process_measure(args[0].clone(), measure);
            if args.len() > 1 {
                size = process_size(args[1].clone());
            } else {
                eprintln!("{}: If the measure is specified, it is also necessary to specify the threshold to be used.", "Invalid threshold".red());
                std::process::exit(1);
            }
            if args.len() > 2 {
                dir = args[2].clone();
            }
        } else { // at least one argument, no measure
            dir = args[0].clone();
        }
    } 
    Arguments{ size:size_in_bytes(&size, &measure), dir:dir } 
}

fn size_in_bytes(size: &u64, measure: &char) -> u64 {
    match measure {
        'k' => size * 1024,
        'm' => size * 1024 * 1024, 
        'g' => size * 1024 * 1024 * 1024, 
        't' => size * 1024 * 1024 * 1024 * 1024,
        _   => { 
            eprintln!("{}: Invalid measure. Must be -k, -m, -g, or -t.", "Invalid measure".red());
            std::process::exit(1);
        }
    }
}

fn process_size(argument: String) -> u64 {
    let size = match argument.parse::<u64>() {
        Ok(n) => n,
        Err(_) => { 
            eprintln!("{}: If the measure is specified, it is also necessary to specify the threshold to be used.", "Invalid threshold".red());
            std::process::exit(1);
        }
    };
    return size;
}

fn process_measure(argument: String, default: char) -> char {
    if argument.starts_with("-m") { 
        return 'm';
    } else if argument.starts_with("-k") {
        return 'k';
    } else if argument.starts_with("-g") {
        return 'g';
    } else if argument.starts_with("-t") {
        return 't';
    } else if argument.starts_with("-") {
        eprintln!("{} specified. Must be -k, -m, -g, or -t.", "Invalid measure".red());
        std::process::exit(1);
    } else {
        return default;
    }
}

fn print_usage() {
    eprintln!("{} - lists files and directories larger than a specified threshold in the specified directory.\n", "bifes".green());
    eprintln!("Usage: bifes <{}> <{}> <{}>", "measure".green(), "threshold".green(), "target".green());
    eprintln!("The {} can be -k, -m, -g, -t for k, mega, giga, or terabytes. If not specified, assumed to be -m. If specified, must also indicate the {} for file size.", "measure".green(), "threshold".green());
    eprintln!("The {} is the minimum number of k, mega, giga, or terabytes that a file or directory must have in order to be listed. This can only be specified with a measure. If not specified, assume to be 1.", "threshold".green());
    eprintln!("The {} is the target directory whose elements will be listed. The sizes of directories within the {} account for all their sub-directories, recursively. If not specified, uses the current directory.", "target".green(), "target".green());
}

