// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let cat = ("Furry McFurson", 3.5);
    //使用模式匹配解构元组并打印出相应的信息
    let (name,age)/* your pattern here */ = cat;

    println!("{} is {} years old.", name, age);
}
