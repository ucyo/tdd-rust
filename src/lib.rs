/*
* Unit tests:
  Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces
  - "unit tests go in the same files as the code"

* Integration tests:
  Integration tests are entirely external to your library and use your code in the same way any other external code
  would, using only the public interface and potentially exercising multiple modules per test
  - "tests directory at the top level of our project directory, next to src"
  - "each file in the tests directory is compiled as its own separate crate"
  See: https://doc.rust-lang.org/book/ch11-03-test-organization.html
*/


#[derive(Debug)]
pub struct Rectangle {
    length: i32,
    width:  i32
}

impl Rectangle {

    pub fn new(length: i32, width: i32) -> Rectangle {
        if length < 0 || width < 0 {
            panic!("Negative values are not accepted! Length {}, Width {}", length, width);
        } else {
            Rectangle {length, width}
        }
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length >= other.length && self.width >= other.width
    }

    pub fn packing(&self, other: &Rectangle) -> Result<bool, String> {
        if self.can_hold(other){
            Ok(true)
        } else {
            Err(String::from("If I can not hold it, I can not pack it"))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;  // scope of test must hold super for all elements

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger  = Rectangle {length: 8, width: 4};
        let smaller = Rectangle {length: 4, width: 3};

        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger  = Rectangle {length: 8, width: 4};
        let smaller = Rectangle {length: 4, width: 3};

        assert!(!smaller.can_hold(&larger))

    }

    #[test]
    #[should_panic(expected = "Negative values")]  // Parts of the panic message is enough
    fn negtive_attributes_are_not_allowed() {
        Rectangle::new(-3, 2);
    }

    #[test]  // TODO: this might not be the proper way to test Result objects
    #[ignore]  // expensive test: cargo test -- --ignored
    fn it_works_result() {
        let larger  = Rectangle {length: 8, width: 4};
        let smaller = Rectangle {length: 4, width: 3};
        let result =  larger.packing(&smaller).unwrap();

        assert!(result)
    }
}
