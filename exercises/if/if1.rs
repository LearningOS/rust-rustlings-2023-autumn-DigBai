// if1.rs
//
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.



pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
     //a>b?a:b//没有return关键字 最后一个表达式会被隐士返回
     //错误提示表明在Rust中，使用 > 和 ? 运算符的组合（a > b ? a : b）是不允许的。正确的写法应该是使用 if-else 来进行条件判断
    if a>b{
        a
    }else{
        b
    }
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
