pub mod frontend;
pub mod intermediate;
pub mod backend;
pub mod runtime;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
