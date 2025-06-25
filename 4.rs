/*
4. Write a program to implement the Scope and Shadowing
*/

fn main() {
    let x = 10;
    {
        let x = 30;
        let y = 20;
        println!("{}",x); //shadowing
        println!("{}",y);
    }
    println!("{}",x);
    //println!("{}",y); //gives error
}
