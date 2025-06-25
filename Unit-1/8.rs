/*
8. Write a program to create an array of 10 elements and implement the following
a. Create a of 2nd and 3rd element
b. Omit the start index of the slice
c. Omit the End Index of the Slice
d. Omit both Start and End Index of the Slice
*/

fn main() {
    let arr: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("array = {:?}", arr);
    let a = &arr[1..3];
    let p = arr.len();
    println!("a = {:?}", a);
    let b = &arr[1..p];
    println!("b = {:?}", b);
    let c = &arr[0..p-1];
    println!("c = {:?}", c);
    let d = &arr[1..p-1];
    println!("d = {:?}", d);
}
