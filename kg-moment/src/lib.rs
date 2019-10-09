pub mod time;


#[cfg(test)]
mod tests {
    use super::time;

    #[test]
    fn it_works() {
        // println!("{:#?}", now());
        let t = time::now();
        println!("{:#?}", t.year);
    }
}
