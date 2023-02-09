use std::fs;

fn main() {
    let path = "tmp/my_items";
    fs::remove_dir_all(path).unwrap();
    fs::create_dir(path).unwrap();
}
