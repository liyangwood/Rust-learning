


pub fn run(){
  let _a:u8 = 1;
  let _b:i8 = 127;

  println!("{}, {}", _a, _b);
}



#[cfg(test)]
mod test {
  #[test]
  fn number_1() {
    assert_eq!(111, 110+1)
  }
}
