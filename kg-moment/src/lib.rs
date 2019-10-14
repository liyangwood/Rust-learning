pub mod moment;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // println!("{:#?}", now());
        let t = moment::now();
        println!("{:#?}", t.get_timestamp());
    }
}
