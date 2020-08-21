extern crate quick_xml;
extern crate serde;
pub mod cda;
pub mod patient;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
