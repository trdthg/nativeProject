// 迭代器

//自定义迭代器
#[derive(Debug)]
struct Counter {
    count: u32
} impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
} impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
#[test]
fn counter_item_test() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
}

#[test]
fn using_other_iterator_trait_methods() {
    for i in vec![1,2,3,4,5] {
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                     .map(|(a, b)| a * b)
                                     .filter(|x| x % i == 0)
                                     .sum();
        // 0  1  2  3  4  5
        // 1  2  3  4  5  0
        // 0  2  6  12 20 0
        // i---筛选后的---   sum
        // 2  2  6  12 20   40
        // 3     6  12      18
        // 4        12 20   32
        println!("---------------------------\n{:?}\n--------------------------", sum);
    }
}
// 第13节 Cacher功能完善
// use std::collections::HashMap;
// struct Cacher2<T>
// where
//     T: Fn(u32) -> u32,
// {
//     calculation: T,
//     map: HashMap<u32, u32>,
// }
// impl<T> Cacher2<T>
// where
//     T: Fn(u32) -> u32
// {
//     fn new(calculation: T) -> Cacher2<T> {
//         let mut map = HashMap::new();
//         Cacher2 { calculation, map }
//     }
//     fn value(&mut self, arg: u32) -> u32 {
//         match &mut self.map.get(&arg) {
//             Some(v) => **v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.map.insert(arg, v);
//                 v
//             }
//         }
//     }
//     fn value2(&mut self, arg: u32) -> &u32 {
//         self.map.entry(arg).or_insert((self.calculation)(arg))
//     }
// }


// struct Cacher3<T, K, V>
// where
//     T: Fn(K) -> V,
// {
//     calculation: T,
//     map: HashMap<K, V>,
// }
// impl<T, K, V> Cacher3<T, K, V>
// where
//     T: Fn(K) -> V,
//     K: Eq + std::hash::Hash + Copy,
// {
//     fn new(calculation: T) -> Cacher3<T, K, V> {
//         let mut map = HashMap::new();
//         Cacher3 { calculation, map }
//     }
//     fn value(&mut self, arg: K) -> V {
//         match &mut self.map.get(&arg) {
//             Some(v) => &v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.map.insert(arg, v);
//                 v
//             }
//         }
//     }
//     fn value2(&mut self, arg: K) -> &V {
//         self.map.entry(arg).or_insert((self.calculation)(arg))
//     }
// }
// #[test]
// fn test1() {
//     let calculation = |num| num * 2;
//     let mut cacher = Cacher2::new(calculation);
//     let a = cacher.value(1);
//     let b = cacher.value(2);
//     println!("{}{}{}{}", a, cacher.value(1), b, cacher.value(2));
// }
// 第12节 重构文件读取

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        // 检查叫做 CASE_INSENSITIVE 的环境变量
        let case_sensitive = std::env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        )
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    // result
    contents.lines()
        .filter(|line| contents.contains(query))
        .collect()
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

pub fn run2(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(config.filename)?;
    let result = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in result {
        println!("{}", line);
    }
    Ok(())
}
// 生命周期参数表明返回值包含contents里字符串slice的引用

// 第11节 测试
// #[cfg(test)]
// mod tests {

//     // common
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }

//     #[test]
//     fn it_works2() {
//         assert_eq!(2 + 2, 5);
//     }

//     #[derive(PartialEq, Debug)]  // 使用assert需要满足实现了PartialEq 和 Debug方法
//     struct Rectangle {
//         width: u32,
//         height: u32,
//     }

//     impl Rectangle {
//         fn can_hold(&self, other: &Rectangle) -> bool {
//             self.width > other.width && self.height > other.height
//         }
//     }

//     // use super::*;

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle { width: 8, height: 7 };
//         let smaller = Rectangle { width: 5, height: 1 };

//         assert!(larger.can_hold(&smaller));
//     }

//     #[test]
//     fn isTwoRecSame() {
//         let a = Rectangle { width: 8, height: 7 };
//         let b = Rectangle { width: 8, height: 7 };
//         assert_eq!(a, b);
//         assert!(a == b);
//         assert!(a != b);
//     }

//     // 自定义测试信息
//     pub fn greeting(name: &str) -> String {
//         // format!("Hello {}!", name)
//         String::from("Hello!")
//     }
//     use super::*;
//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("Carol");
//         assert!(
//             result.contains("Carol"),
//             "Greeting did not contain name, value was `{}`", result
//         );
//     }

//     // 检查panic是否正常
//     pub struct Guess {
//         value: i32,
//     }
//     impl Guess {
//         pub fn new(value: i32) -> Guess {
//             if value < 1 {
//                 panic!("Guess value must be greater than or equal to 1, got {}.",
//                        value);
//             } else if value > 100 {
//                 panic!("Guess value must be less than or equal to 100, got {}.",
//                        value);
//             }
//             Guess {
//                 value
//             }
//         }
//     }

//     #[test]
//     #[should_panic]
//     fn any_panic() {
//         Guess::new(200);
//     }

//     #[test]
//     #[should_panic(expected = "Guess value must be less than or equal to 100")]
//     fn greater_than_100() {
//         Guess::new(200);  // 不符合，报错不
//     }
//     #[test]
//     #[should_panic(expected = "Guess value must be less than or equal to 100")]
//     fn greater_than_100_2() {
//         Guess::new(0);  // 这样虽然也回报错，但是不符合expect，所以不通过
//     }

//     #[test]
//     fn it_works3() -> Result<(), String> {
//         if 2 + 2 == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus two does not equal four"))
//         }
//     }
// }

// 1. cargo test
// 2. cargo test xxx 指定运行
// 3. cargo test xxx 指定运行
// 注意输出中不会出现测试通过时打印的内容，即 I got the value 4。因为当测试通过时，这些输出会被截获。失败测试的输出 I got the value 8 ，则出现在输出的测试摘要部分，同时也显示了测试失败的原因。
// 4. 如果你希望也能看到通过的测试中打印的值，截获输出的行为可以通过 --nocapture 参数来禁用：
// fn prints_and_returns_10(a: i32) -> i32 {
//     println!("I got the value {}", a);
//     10
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn this_test_will_pass() {
//         let value = prints_and_returns_10(4);
//         assert_eq!(10, value);
//     }

//     #[test]
//     fn this_test_will_fail() {
//         let value = prints_and_returns_10(8);
//         assert_eq!(5, value);
//     }
//     #[test]
//     #[ignore]
//     fn this_test_will_beignored() {
//         let value = prints_and_returns_10(8);
//         assert_eq!(5, value);
//     }
// }

// 测试私有模块
// pub fn add_two(a: i32) -> i32 {
//     internal_adder(a, 2)
// }

// fn internal_adder(a: i32, b: i32) -> i32 {
//     a + b
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn internal() {
//         assert_eq!(4, internal_adder(2, 2));
//     }
// }
