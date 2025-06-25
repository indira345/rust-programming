/*
2. Write a program to display Output following pattern using Placeholders
1
22
333
4444
55555
*/

fn main() {
    println!("{}",1);
    println!("{0}{0}",2);
    println!("{0}{0}{0}",3);
    println!("{0}{0}{0}{0}",4);
    println!("{0}{0}{0}{0}{0}",5);
}
