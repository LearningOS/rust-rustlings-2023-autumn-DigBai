// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let mut x = 100;
    // Rust 中不能同时拥有多个可变引用指向同一数据
    //在不同的作用域内创建了两个可变引用，确保了在同一时间只有一个可变引用存在
    // let y = &mut x;
    // let z = &mut x;
    // *y += 100;
    // *z += 1000;
    let y = &mut x;
    //可变引用创建完要在下一个可变引用使用前结束当前的使用
    *y += 100;
    let z = &mut x;
    
    *z += 1000;
    assert_eq!(x, 1200);
}
