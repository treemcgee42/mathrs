pub mod mstructs;
pub mod polynomials;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        assert_eq!("5".parse::<f64>(),Ok(5.0_f64));
    }
}
