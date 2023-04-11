//#[macro_use] extern crate impl_ops;
//#[macro_use]
//extern crate approx;

pub mod tuples;
//pub mod foo;
//pub mod bar;
//pub mod baz;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
