/*
8. Write a program to Print the following patterns
&
&&
&&&
&&&&
&&&&&
*/
fn main() {
    for i in 0..6 {
        for j in 0..i {
            print!("&");
        }
        println!("");
    }
}
