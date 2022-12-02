include!("../src/main.rs");

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        println!("{:#?}", *crate::APP_CONFIG);
    
    }
}