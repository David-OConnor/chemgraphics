// We use mathematical conventions that may direct upper-case var names,
// or lower-case constants.
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(non_ascii_idents)]
#![feature(vec_remove_item)]
#![feature(const_vec_new)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
