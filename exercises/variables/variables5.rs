// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.



fn main() {
    //number被赋值成字符串了
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    //不能加mut 因为不让改
    //所以只能再生命一次let
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
