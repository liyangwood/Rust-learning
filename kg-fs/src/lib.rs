pub mod path;
pub mod dir;


#[cfg(test)]
mod tests {
    use super::dir;
    use std::path::Path;

    #[test]
    fn test_dir_create(){
        let path = Path::new(".data/aaa/bbb");
        dir::create(path, false).unwrap();

        assert!(path.exists());  
        dir::remove(path);
        assert_eq!(path.exists(), false);
    }

    #[test]
    fn test_dir_remove(){
       
    }
}
