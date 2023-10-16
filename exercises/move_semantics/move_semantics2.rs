// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let vec0 = Vec::new();
   //在 Rust 中，一旦将值移动给另一个变量或函数后，原始的变量将不能再使用，除非通过引用方式借用。
   //let mut vec1 = fill_vec(vec0);这样就把vec0的值移动给函数了  所以改成不vec0的clone移动给他
    let mut vec1 = fill_vec(vec0.clone());

    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);
 

    vec1.push(88);

    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
