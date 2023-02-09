use std::fs;

fn main() {
    let path = "../storage";
    fs::remove_dir_all(path).unwrap();
    fs::create_dir(path).unwrap();
}
