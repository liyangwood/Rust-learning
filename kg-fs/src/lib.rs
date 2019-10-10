macro_rules! err {
    ($text:expr, $kind:expr) => {
        return Err(Error::new($kind, $text));
    };

    ($text:expr) => {
        err!($text, ErrorKind::Other)
    };
}

mod error;
pub mod path;
pub mod dir;
pub mod file;

use crate::error::{Error, ErrorKind};




#[cfg(test)]
mod tests {
    // use super::dir;
    use super::*;
    use std::path::Path;

    #[test]
    #[ignore]
    fn test_dir(){
        let path = Path::new(".data/aaa/bbb");
        dir::create(path, false).unwrap();

        assert!(path.exists());  
        dir::remove(path).unwrap();
        assert_eq!(path.exists(), false);
    }

    #[test]
    fn test_file(){
        let path = Path::new(".data/test.file");

        file::remove(&path).unwrap();
        assert_eq!(path.exists(), false);

        let str = "jacky.li";
        file::write_file(&path, &str).unwrap();

        let rs = file::read_file(&path).unwrap();
        assert_eq!(rs, str.to_string());
    }
}
