// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}
//Rust 预期 current_favorite_color 返回一个 String 类型，但实际上它返回了一个字符串字面值，即 &str 类型。
//为了修复这个错误，你可以将字符串字面值转换为 String 类型，如下所示：
fn current_favorite_color() -> String {
    "blue".to_string()
}
