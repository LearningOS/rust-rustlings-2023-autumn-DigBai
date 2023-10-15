// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.



fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        //.iter()：用于对集合创建一个不可变的迭代器，允许读取集合中的元素。
        //.iter_mut()：用于对集合创建一个可变的迭代器，允许读取或修改集合中的元素。
        //.into_iter()：用于将集合转换为迭代器，在迭代过程中拥有集合所有权，允许读取或移动集合中的元素。
        //*element += 1; 表示将 Vec 中当前元素的值加一
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element*=2
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v//每个元素都乘以2之后返回v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        //.map() 方法，它接受一个闭包作为参数。这个闭包（由 |element| 定义）对迭代器的每个元素执行某种操作
        //参数 element，用于表示迭代器中的每个元素。您可以在闭包中对这个元素执行任何操作。
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        element*2
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        //1.. 创建了一个无限的整数迭代器，从1开始，一直往后递增 .filter(|x| x % 2 == 0) 对迭代器应用一个过滤器，只保留偶数
        //.take(5) 从过滤后的迭代器中取前5个元素 collect将迭代器的元素收集到一个集合中，
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();//v=[2,4,6,8,10]
        let ans = vec_loop(v.clone());//调用名为 vec_loop 的函数，传递了 v 的克隆作为参数，并将结果赋给 ans 变量。
        //ans=[48,12,16,20]
        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());//v.iter().map(|x| x * 2).collect::<Vec<i32>>()对v
        //闭包每一个元素乘2 存造一个新的Vec<i32>中
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
