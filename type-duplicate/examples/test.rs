extern crate type_duplicate_derive;

use type_duplicate_derive::Duplicate;

#[derive(Duplicate)]
pub struct SomeStruct;

fn main() {
    let test = SomeStructBson { a: 42, b: 42 };

    println!("{:?}", test);
}
