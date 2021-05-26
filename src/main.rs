// small tutorial from https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

const MAX_POINTS: u32 = 100_000; //different from not mutable variable: main
                                 //main to tell other programmer that it is a constant var



fn main() {
    // without mut you receive a compiling error!
    let x = MAX_POINTS;
    println!("The value of x is: {}", x);
    // this is call shadowing. perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

    //shadowing can be used to change type
    let x = "";
    let x = x.len();
    println!("The value of x is: {}", x);

    //cannot assign a different type to a mut variable!!!
}