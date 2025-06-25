/*
7. Write a program to create and access a tuple.
*/

fn main() {
    let t: (i32, &str) = (30, "rust");
    println!("tuple = {:?}", t);
}
