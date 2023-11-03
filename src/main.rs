/// This is a program that will replace all spaces in filenames with underscores for all files in a directory.
fn main() {
    let path: &'static str = "/absolute/path/to/directory/with/files/to/rename";
    let directory_with_files_to_rename = std::fs::read_dir(path).unwrap();
    for file in directory_with_files_to_rename {
        let file = file.unwrap();
        let path = file.path();
        let new_path = path.to_str().unwrap().replace(" ", "_");
        println!("Renaming {:?} to {:?}", path, new_path);
        // When you're ready to actually rename things, uncomment the line below.
        // std::fs::rename(path, new_path).unwrap();
    }
}
