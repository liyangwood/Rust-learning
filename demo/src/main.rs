use kg_fs::{dir, file};
use std::path::Path;

fn main() {
    let path = Path::new(".data/aaa/test.txt");

    dir::create(Path::new(".data/aaa"), true).unwrap();

    file::write_file(&path, "aaasdfsdfs").unwrap();
}
