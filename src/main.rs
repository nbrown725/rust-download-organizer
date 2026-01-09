use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::env::args;

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

    let home_dir = PathBuf::from("/home/nitechno/");
    let source_dir;
    let user_arg = args().skip(1).next();
    match user_arg {
        Some(arg) => source_dir = PathBuf::from(arg),
        None => source_dir = PathBuf::from("/home/nitechno/Downloads/"),
    }

    let files = fs::read_dir(&source_dir).expect("Error: Failed to read Download directory");

    for f in files {
        let file = f.expect("Error: Failed to read file");

        // Make sure it's a file (not a directory)
        if file.file_type().expect("Error: Failed to check file type").is_file() {

            // Normalize file name to lowercase
            let name_string = file.file_name().into_string().expect("Error: Failed to parse filename into String");

            // Make sure there's some '.ext'
            if (&name_string).contains(".") {

                // Ext after last '.'
                let ext = (&name_string).split(".").last().expect("Error: Failed to split filename String");

                // Get path for relative ext destination
                let ext_dir = ext_map.get(ext);
                // Get path for absolute ext destination
                let destination_dir;
                match ext_dir {
                    Some(p) => destination_dir = home_dir.join(p),
                    None => continue,
                }

                // If path does not exist, create new directory
                if !fs::exists(&destination_dir).expect("Error: Failed checking if path exists") {
                    fs::create_dir_all(&destination_dir).expect("Error: Failed to create necessary directory");
                }

                // Get destination path, including filename
                let destination = destination_dir.join(&name_string);
                let source = source_dir.join(&name_string);

                // If the file does NOT exist in destination path, move it
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

                        println!("{:?}", new_destination);
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
        }
    }
}