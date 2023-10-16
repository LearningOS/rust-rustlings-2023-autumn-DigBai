// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let  data = "Rust is great!".to_string();

    get_char(&data);//加上&表示只是引用 避免更改所有权
////&是引用的意思 如果你不加 &，那么在函数调用中就会传递 data 的所有权，而不是引用。
//加上&代表是引用 不是所有权  因为下面对data进行了了修改 所以要加上mut表示是可变的引用
//在你的代码中，data 在调用 string_uppercase 函数时作为参数传递，这将转移 data 的所有权给函数。
//然而，在之后的代码中又尝试对 data 进行访问，*data = new_data;这是不允许的，因为所有权已经在函数调用中移动了。

//为了解决这个问题，你需要确保在调用 string_uppercase 后不再使用 data。如果你需要在之后继续使用它，
//你可以在函数调用之前对 data 进行克隆（clone），这样就会保留原始 data 的所有权。
let mut mdata=data.clone();
    string_uppercase(&mut mdata);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase( data: &mut String) {
    //String表示拥有所有权的字符串类型 变量可以使用也可以修改 &String表示应用一个字符串，只能使用不能修改
    //函数 main 可以使用但不能修改 data 指向的 String 内容。
    //因为data是引用的String类型 不能修改 所以to_uppercase大写就不行 改成& mut String
    //然后临时值通常指的是临时创建的、不被绑定到变量的值data.to_uppercase() 是一个临时值
    //所以临时值赋给 data 引用，但由于引用的生命周期限制，这是不允许的。
    // data = data.to_uppercase();data是mut可变的类型是可变的string引用是mut String类型的 而data.to_uppercase()返回的是String类型的 两个赋值不匹配
    //所以用*data data是引用 *data是解引用 就是原始的data 没有引用 这个let data = "Rust is great!".to_string();
    //data = data.to_uppercase();//data 是 "hello"，那么 data.to_uppercase() 将返回一个新的 String，内容为 "HELLO"，而不会改变 data 的内容。
  
    println!("{}",  data.to_uppercase());
}
