// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.

pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        //let identifier = if animal == "crab" 如果 animal 的值等于 "crab"，则将 1.0 赋给变量 identifier
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        0
    };

    // DO NOT CHANGE THIS STATEMENT BELOW、
    //if语句的值返回给habitat
    let habitat = if identifier == 1 {//上面的判断中identifier可能是1,2.0,3，unknow因为只有一个可能所以不会出现类型不一致
        "Beach"
    } else if identifier == 2 {//但是identifier等于2.0会与2冲突
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    };

    habitat//隐士返回最后一个表达式的值
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
