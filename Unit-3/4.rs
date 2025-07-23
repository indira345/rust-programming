/*
4. Write a program to Match a pattern using If Let Expression.
*/

fn main() {  
    let c = ("Rust", "programming","course");
    if let ("Rust", "programming","course") = c {
        println!("all values matched with the expression");
    } else {
        println!("Value unmatched");
    }
}
