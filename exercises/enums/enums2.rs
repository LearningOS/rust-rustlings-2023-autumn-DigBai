// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.


#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move { x: i32, y: i32 },           // Variant with struct
    Echo(String),                      // Variant with a String
    ChangeColor(u8, u8, u8),           // Variant with RGB color values
    Quit,                 
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}
//定义一个枚举类 这个枚举类里面有4中类型
//枚举类有一个声明call函数
//let messages创建一个数组，第一项是MOve 第二项是echo
//对数组的每一项 执行call
fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
