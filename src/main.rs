// use clap::command;
// use clap::Arg;
// use dirs;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None) ]
struct Args {
    #[arg(short, long)]
    new_file: Option<String>,
    existing_file: Option<String>,
}

fn main() {
    let mut storage_dir = dirs::home_dir().unwrap();
    storage_dir.push(".h-files");

    let args = Args::parse();
    match (args.new_file, args.existing_file) {
        (None, Some(name)) => {
            show_file(storage_dir, name);
        }
        (Some(name), None) => {
            make_new_file(storage_dir, name.clone());
        }
        (Some(_), Some(_)) => {
            println!(
                "Got both a new and existing file. You can only send one. Check --help for details"
            );
        }
        _ => {
            list_files(storage_dir);
        }
    }
}

fn make_new_file(storage_dir: PathBuf, name: String) {
    let mut file_path = storage_dir.clone();
    file_path.push(name.clone());
    file_path.set_extension("txt");
    if file_path.exists() {
        println!("File {} already exists", name.clone());
    } else {
        let text = String::from("put your notes here");
        fs::write(file_path, text).unwrap();
        println!("Created file: {}", name.clone());
    }
}

fn show_file(storage_dir: PathBuf, name: String) {
    let mut file_path = storage_dir.clone();
    file_path.push(name.clone());
    file_path.set_extension("txt");
    if file_path.exists() {
        let text = fs::read_to_string(file_path).unwrap();
        println!("-----------------------------------------");
        println!("{}", text);
        println!("-----------------------------------------");
    } else {
        println!("No file for: {}", name.as_str());
    }
}

// fn main() {
//     let mut storage_dir = dirs::home_dir().unwrap();
//     storage_dir.push(".h-files");
//     match verify_dir(&storage_dir) {
//         true => {
//             let matches = command!().arg(Arg::new("file")).get_matches();
//             match matches.get_one::<String>("file") {
//                 Some(f) => {
//                     let mut file_path = storage_dir.clone();
//                     file_path.push(f);
//                     file_path.set_extension("txt");
//                     if file_path.exists() {
//                         let text = fs::read_to_string(file_path).unwrap();
//                         println!("-----------------------------------------");
//                         println!("{}", text);
//                         println!("-----------------------------------------");
//                     } else {
//                         println!("No file for: {}", f);
//                     }
//                 }
//                 None => {
//                     println!("Existing files:");
//                     list_files();
//                 }
//             }
//         }
//         false => {
//             println!("Could not make storage directory");
//             println!("{}", storage_dir.display());
//         }
//     }
// }

// fn verify_dir(dir: &PathBuf) -> bool {
//     if dir.exists() {
//         true
//     } else {
//         match fs::create_dir_all(dir) {
//             Ok(_) => true,
//             Err(_) => false,
//         }
//     }
// }

pub fn list_files(storage_dir: PathBuf) {
    for entry in WalkDir::new(storage_dir)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.path().is_file() {
            println!("{}", entry.path().file_stem().unwrap().to_str().unwrap());
        }
    }
}
