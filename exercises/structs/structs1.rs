// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.



struct ColorClassicStruct {
    // TODO: Something goes here
}

struct ColorTupleStruct(/* TODO: Something goes here */);
struct color{
    red:u8,//无符号8位整数 类似于 int red 
    green:u8,
    blue:u8,
}
struct color2(u8,u8,u8);//使用元组
#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        // let green =
let green=color{//设置green是一个color结构体 red green blue是结构体的属性
    red:0,
    green:255,
    blue:0
};
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        // let green =
        let green=color2(0,255,0);
        assert_eq!(green.0, 0);//green.1 就是访问结构体 green 的第二个字段。
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        // let unit_like_struct =
        let unit_like_struct =UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);//表达式的目的是将单元结构体的调试格式输出插入到字符串模板中，以创建一个描述单元结构体的字符串

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
