// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1;
    //numbers.0 表示访问元组的第一个元素。
    //numbers.1 表示访问元组的第二个元素。
    //numbers.2 表示访问元组的第三个元素。

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")//"This is not the 2nd number in the tuple!"一个错误信息，用于在测试失败时显示给用户。在这种情况下
}
