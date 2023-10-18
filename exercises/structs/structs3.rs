// structs3.rs
//
// Structs contain data, but can also have logic. In this exercise we have
// defined the Package struct and we want to test some logic attached to it.
// Make the code compile and the tests pass!
//
// Execute `rustlings hint structs3` or use the `hint` watch subcommand for a
// hint.


#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: i32,
}

//impl（是 "implementation" 的缩写）是 Rust 编程语言中的一个关键字，用于为结构体、枚举、特质（traits）或其他类型添加方法的实现。在您的代码示例中，impl Package 用于实现与 Package 结构体相关的方法，这些方法可以在结构体的实例上调用。

//具体来说，在您的代码中，impl Package 用于实现以下方法：

//fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package：这是一个构造函数，用于创建新的 Package 结构体实例。它接受三个参数并返回一个新的 Package 实例。这个方法用于初始化包裹对象。

//fn is_international(&self) -> bool：这是一个用于检查包裹是否国际的方法。它接受 self 作为参数，其中 self 表示当前实例，然后检查寄件国家和收件国家是否相同，如果不同，则返回 true 表示国际包裹，否则返回 false 表示本地包裹。

//fn get_fees(&self, cents_per_gram: i32) -> i32：这个方法用于计算运输费用，它接受每克的费用 cents_per_gram 作为参数，并根据包裹的重量计算总费用。它也接受 self 作为参数，以表示当前包裹实例。
impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
        if weight_in_grams <= 0 {
            panic!("Can not ship a weightless package.")
        } else {
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    fn is_international(&self) -> bool {
        // Something goes here...
        //  // 检查寄件国家和收件国家是否相同，如果不同则是国际包裹
        self.sender_country!=self.recipient_country
    }

    fn get_fees(&self, cents_per_gram: i32) -> i32 {
        //// 根据包裹重量和每克的费用计算运输费用
        self.weight_in_grams*cents_per_gram
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, -2210);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
