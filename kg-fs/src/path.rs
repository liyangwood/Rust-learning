// use std::path::Path;
use std::env;
// use std::option;


pub fn pwd()->String{
  let rs = env::current_dir().expect("error in path.rs");
  println!("The current directory is {}", rs.display());
  
  return rs.to_str().expect("error in path.rs").to_string();
}

#[cfg(test)]
mod tests {
  use super::*;

    #[test]
    fn it_works() {
      let path = pwd();
      println!("{}", path);
      // assert_eq!("", path);

    }
}