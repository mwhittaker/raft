#![crate_type = "lib"]

pub mod foo;
pub mod bar;
pub mod baz;

fn main() {
    foo::moo();
    bar::moo();
    baz::moo();
}
