#![feature(box_syntax,plugin)]
#![plugin(interpolate_idents)]

#[macro_use] extern crate av;
#[macro_use] extern crate nom;
extern crate flavors;

mod flv;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
