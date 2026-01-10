use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::env::args;
use std::env::var;

fn move_file(source: &PathBuf, destination: &PathBuf, name_string: &String, destination_dir: &PathBuf) {
    if !fs::exists(&destination).expect("Error: Failed checking if copy exists") {
        fs::rename(&source, &destination).expect("Error: Failed to move file");
        println!("Moved {} → {}", source.to_str().unwrap(), destination.to_str().unwrap());
    } else {
        let mut rename = true;
        let mut i = 1;

        while rename {

            // Split file name and extension
            let (old_name, old_ext) = (&name_string).rsplit_once('.').unwrap();
            // Add `_(i)` to the end of file name
            let new_name = format!("{}_({}).{}", old_name, i, old_ext);
            let new_destination = &destination_dir.join(new_name);

            if !fs::exists(&new_destination).expect("Error: Failed checking if copy exists") {
                fs::rename(&source, &new_destination).expect("Error: Failed to move file");
                println!("Moved {} → {}", source.to_str().unwrap(), new_destination.to_str().unwrap());
                rename = false;
            } else {
                i += 1;
            }
        }
    }
}

fn check_file(file: &fs::DirEntry, name_string: &String) -> bool {
    if file.file_type().expect("Error: Failed to check file type").is_file() {
    
    if name_string.ends_with(".part") || name_string.ends_with(".crdownload") {
            return false;
        }

        // Check for placeholder metadata
        if let Ok(metadata) = file.metadata() {
            if metadata.len() == 0 {
                return false;
            }
        }
        
        if name_string.contains(".") {
            return true;
        }
    }

    return false;
}

fn process_file(file: fs::DirEntry, ext_map: &HashMap<&str, &str>, source_dir: &PathBuf, home_dir: &PathBuf) {

    let name_string = file.file_name().into_string().expect("Error: Failed to parse filename into String");
    
    if check_file(&file, &name_string) {
        // Get extension type
        let ext = (&name_string).split(".").last().expect("Error: Failed to split filename String");

        // Get destination directory
        let destination_dir = match ext_map.get(ext) {
            Some(p) => home_dir.join(p),
            None => return,
        };

        // Create destination directory if absent
        if !fs::exists(&destination_dir).expect("Error: Failed checking if path exists") {
            fs::create_dir_all(&destination_dir).expect("Error: Failed to create necessary directory");
        }

        // Get destination and source paths
        let destination = destination_dir.join(&name_string);
        let source = source_dir.join(&name_string);

        
        move_file(&source, &destination, &name_string, &destination_dir);
    }
}

fn main() {
    
    // Directories and the extensions they map to
    let extensions: HashMap<&str, Vec<&str>> = HashMap::from([
        ("Pictures", Vec::from(["jpg", "jpeg", "png", "gif", "webp", "svg", "tiff", "heif", "psd", "raw", "bmp", "ico", "ai"])),
        ("Videos", Vec::from(["mp4", "mov", "webm", "mkv", "flv", "ogg", "avi", "m4p", "mv4", "mpg", "mpeg"])),
        ("Music", Vec::from(["mp3", "wav", "aiff", "flac", "ogg", "opus"])),
        ("Documents", Vec::from(["pdf", "docx", "pptx", "csv"]))
    ]);

    // Map extensions to directories for O(1) lookup
    let mut ext_map: HashMap<&str, &str> = HashMap::new();

    for e in extensions {
        let (path, ext_list) = e;

        for ext in ext_list {
            ext_map.insert(ext, path);
        }
    }

    let home_dir = match var("HOME") {
        Ok(val) => PathBuf::from(val),
        Err(e) => {
            println!("Error: Failed to get home directory. Stopping. {}", e);
            return;
        },
    };


    let source_dir = match args().skip(1).next() {
        Some(arg) => PathBuf::from(arg),
        None => home_dir.join("Downloads"),
    };

    let files = fs::read_dir(&source_dir).expect("Error: Failed to read Download directory");

    for f in files {
        if let Ok(file) = f {
            process_file(file, &ext_map, &source_dir, &home_dir);
        }
    }
}