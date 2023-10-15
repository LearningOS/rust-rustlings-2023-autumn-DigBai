// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num//函数体最后加了一个分号，导致函数隐式返回了 ()，而不是 i32
    //在Rust中，分号表示语句的结束，但也可以用于表达式，此时它将表达式转变为语句并返回 ()，即 unit 类型。这是Rust的设计选择，分号告诉编译器这是一个语句，而不是一个表达式，因此会返回 unit 类型。
//在函数体末尾加上分号将表达式变为语句，这意味着它不会返回值
//在Rust中，分号表示语句的结束，但也可以用于表达式，此时它将表达式转变为语句并返回 ()，即 unit 类型。这是Rust的设计选择，分号告诉编译器这是一个语句，而不是一个表达式，因此会返回 unit 类型。
//在函数体末尾加上分号将表达式变为语句，这意味着它不会返回值

}
