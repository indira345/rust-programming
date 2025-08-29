/*
6. Write a program to Multiplication Table using Loop Labels
*/
fn main() {
 for i in 1..5{ //outer loop
  println!("Multiplication Table of : {}", i);
   for j in 1..11 { // inner loop
       println!("{} * {} = {}", i, j, i * j);
   }
 }
}
