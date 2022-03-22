// 绑定和可变性
// 🌟 变量只有在初始化后才能被使用
//
// 修复下面代码的错误并尽可能少的修改
// fn main() {
//     let x: i32 = 5; // 未初始化，但被使用
//     let _y: i32 = 5; // 未初始化，也未被使用
//     println!("{} is equal to 5", x);
// }

//🌟🌟 可以使用 mut 将变量标记为可变
// // 完形填空，让代码编译
// fn main() {
//     let mut x = 1;
//     x += 2;
//
//     println!("{} = 3", x);
// }

// 修复下面代码的错误并使用尽可能少的改变
// fn main() {
//     let x: i32 = 10;
//     {
//         let y: i32 = 5;
//         println!("x 的值是 {}, y 的值是 {}", x, y);
//     }
//     println!("x 的值是 {}", x);
// }

// 修复错误
// fn main() {
//     let x = "hello";
//     println!("{}, world", x);
// }

// 变量遮蔽( Shadowing )
// 🌟🌟 若后面的变量声明的名称和之前的变量相同，则我们说：第一个变量被第二个同名变量遮蔽了( shadowing )
// 只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)
//TODO:不明白assert_eq!
// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, x);
//     }
//     assert_eq!(12, 12);
//     let x = 42;
//     println!("{}", x); // 输出 "42".
// }

//🌟🌟 删除一行代码以通过编译
// fn main() {
//     let mut x: i32 = 1;
//     x = 7;
//     // 遮蔽且再次绑定
//     let _x = x;
//     //x += 3;
//
//
//     let _y = 4;
//     // 遮蔽
//     let _y = "I can also be bound to text!";
// }

// 未使用的变量
// 使用以下方法来修复编译器输出的 warning :
// fn main() {
//     let _x = 1;
// }

// compiler warning: unused variable: `x`

// 变量解构
// 🌟🌟 我们可以将 let 跟一个模式一起使用来解构一个元组，最终将它解构为多个独立的变量
// 修复下面代码的错误并尽可能少的修改
// fn main() {
//     let (mut x, y) = (1, 2);
//     x += 2;
//
//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
// }
// 解构式赋值
// 该功能于 Rust 1.59 版本引入：你可以在赋值语句的左式中使用元组、切片或结构体进行匹配赋值。
//todo:
// fn main() {
//     let (x, y);
//     (x, ..) = (3, 4);
//     [.., y] = [1, 2];
//     // 填空，让代码工作
//     assert_eq!([x, y], __);
// }


// 数值类型
// 整数
// 移除某个部分让代码工作
// fn main() {
//     let x: i32 = 5;
//     let mut y: u32 = 5;
//
//     y = x as u32;
//
//     let z = 10; // 这里 z 的类型是?
//     println!("hello,world!");
// }


// 填空
// fn main() {
//     let _v: u16 = 38_u8 as u16;
// }


//  修改 `assert_eq!` 让代码工作
// fn main() {
//     let x = 5;
//     assert_eq!("i32".to_string(), type_of(&x));
// }
//
// // 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }


// 填空，让代码工作
// fn main() {
//     assert_eq!(i8::MAX, 127);
//     assert_eq!(u8::MAX, 255);
// }


// 解决代码中的错误和 `panic`
// fn main() {
//     let v1 = 247_u8 + 8;
//     let v2 = i8::checked_add(119, 8).unwrap();
//     println!("{},{}",v1,v2);
// }


// 修改 `assert!` 让代码工作
// fn main() {
//     let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
//     assert_eq!(v, 1597);
// }

//浮点数
// 将 ? 替换成你的答案
// fn main() {
//     let x = 1_000.000_1; // f64
//     let y: f32 = 0.12; // f32
//     let z = 0.01_f64; // f64
// }

// 🌟🌟 使用两种方法来让下面代码工作
// fn main() {
//     assert_eq!(0.1_f32 + 0.2_f32, 0.3_f32);
// }

// 序列Range
// 🌟🌟 两个目标: 1. 修改 assert! 让它工作 2. 让 println! 输出: 97 - 122
// fn main() {
//     let mut sum = 0;
//     for i in -3..2 {
//         sum += i
//     }
//     assert_eq!(sum, -5);
//     for c in 'a'..='z' {
//         let c = c as u8;
//         println!("{}", c);
//     }
// }

// 填空
// use std::ops::{Range, RangeInclusive};
//
// fn main() {
//     assert_eq!((1..5), Range { start: 1, end: 5 });
//     assert_eq!((1..=5), RangeInclusive::new(1, 5));
// }

// 填空，并解决错误
//todo:位操作不明白
// fn main() {
//     // 整数加法
//     assert_eq!(1u32 + 2, 3);
//     // 整数减法
//     assert_eq!(1i32 - 2, -1);
//     assert_eq!(1i8 - 2, -1);
//     assert_eq!(3 * 50, 150);
//     assert_eq!(9.6f32 / 3.2, 3.0); // error ! 修改它让代码工作
//     assert_eq!(24 % 5, 4);
//     // 逻辑与或非操作
//     assert!(true && false == false);
//     assert!(true || false == true);
//     assert_eq!(!true, false);
//     // 位操作
//     println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
//     println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
//     println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
//     println!("1 << 5 is {}", 1u32 << 5);
//     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
// }

// 字符、布尔、单元类型
// 字符
// use std::mem::size_of_val;
//
// fn main() {
//     let c1 = 'a';
//     //占用的字节量
//     assert_eq!(size_of_val(&c1), 4);
//     let c2 = '中';
//     assert_eq!(size_of_val(&c2), 4);
//     println!("Success!")
// }

// fn main() {
//     let c1 = "中";
//     print_char(&c1);
// }
//
// fn print_char(c : &str) {
//     println!("{}", c);
// }

//布尔
// make  println! work
// fn main() {
//     let _f: bool = false;
//     let t = true;
//     if t {
//         println!("Success!")
//     }
// }

// fn main() {
//     let f = true;
//     let t = false;
//     assert_eq!(t, !f);
//
//     println!("Success!")
// }

//单元类型
// 让代码工作，但不要修改 `implicitly_ret_unit` !
// fn main() {
//     let v0: () = ();
//     let v = (2, 3);
//     assert_eq!(v0, implicitly_ret_unit());//空和空
//     println!("Success!")
// }
//
// fn implicitly_ret_unit() {
//     println!("I will returen a ()")
// }
//
// // 不要使用下面的函数，它只用于演示！
// fn explicitly_ret_unit() -> () {
//     println!("I will returen a ()")
// }

//单元类型占用的内存大小是多少？
// 让代码工作：修改 `assert!` 中的 `4`
// use std::mem::size_of_val;
//
// fn main() {
//     let unit: () = ();
//     assert_eq!(size_of_val(&unit), 0);//空元组占用字节为0
//
//     println!("Success!")
// }

//语句与表达式
// fn main() {
//     let x = 5u32;
//     let y = {
//         let x_squared = x * x;
//         let x_cube = x_squared * x;
//         // 下面表达式的值将被赋给 `y`
//         x_cube + x_squared + x
//     };
//     let z = {
//         2 * x;// 分号让表达式变成了语句，因此返回的不再是表达式 `2 * x` 的值，而是语句的值 `()`
//         1
//     };
//     println!("x is {:?}", x);
//     println!("y is {:?}", y);
//     println!("z is {:?}", z);
// }

//练习
// 使用两种方法让代码工作起来
// 使用两种方法让代码工作起来
// fn main() {
//     let v = {
//         let x = 1;
//         x + 2
//     };
//     assert_eq!(v, 3);
// }

// fn main() {
//     let v = {
//         let x = 3;
//         x
//     };
//     assert_eq!(v, 3);
// }

// fn main() {
//     let s = sum(1, 2);
//     assert_eq!(s, 3);
// }
//
// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }

//函数
// fn main() {
//     // 不要修改下面两行代码!
//     let (x, y) = (1, 2);
//     let s = sum(x, y);
//
//     assert_eq!(s, 3);
// }
//
// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }
// fn main() {
//     print();
// }
//
// // 使用另一个类型来替代 i32
// fn print() -> () {
//     println!("hello,world");
// }

//todo:搞不明白
// 用两种方法求解
// fn main() {
//     never_return();
// }
// fn never_return() -> ! {
//     // 实现这个函数，不要修改函数签名!
// }

//所有权
// fn main() {
//     // 使用尽可能多的方法来通过编译
//     let x = String::from("hello, world");
//     let y = &x;
//     println!("{},{}",x,y);
// }
// fn main() {
//     // 使用尽可能多的方法来通过编译
//     let x = "hello,world";
//     let y = x;
//     println!("{}   {}",x,y);
// }
// fn main() {
//     // 使用尽可能多的方法来通过编译
//     let x = &String::from("hello, world");
//     let y = x;
//     println!("{},{}", x, y);
// }
// fn main() {
//     // 使用尽可能多的方法来通过编译
//     let x = 10;
//     let y = x;
//     println!("{},{}",x,y);
// }

// 不要修改 main 中的代码
// fn main() {
//     let s1 = String::from("hello, world");
//     let s2 = take_ownership(s1);
//
//     println!("{}", s2);
// }
//
// // 只能修改下面的代码!
// fn take_ownership(s: String) -> String {
//     s
// }

// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }
//
// // 只能修改下面的代码!
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     // convert String to Vec
//     // 将 String 转换成 Vec 类型
//     let x = s.as_bytes();
//     println!("{:?}",x);
//     s
// }

// 修复错误，不要删除任何代码行
// fn main() {
//     let s = String::from("hello, world");
//     print_str(&s);
//     println!("{}", s);
// }
//
// fn print_str(s: &String) {
//     println!("{}", s)
// }
// 修复错误，不要删除任何代码行
// fn main() {
//     let s = String::from("hello, world");
//     print_str(s.clone());
//     println!("{}", s);
// }
//
// fn print_str(s: String)  {
//     println!("{}",s)
// }
// 不要使用 clone，使用 copy 的方式替代
// fn main() {
//     let x = (1, 2, (), "hello".to_string());
//     let y = &x;
//     println!("{:?}, {:?}", x, y);
// }

// 可变性
// 当所有权转移时，可变性也可以随之改变。
// fn main() {
//     let s = String::from("hello, ");
//     // 只修改下面这行代码 !
//     let mut s1 = s;
//     s1.push_str("world")
// }

// fn main() {
//     let x = Box::new(5);
//
//     let mut y = Box::new(3);    // 完成该行代码，不要修改其它行！
//
//     *y = 4;
//
//     assert_eq!(*x, 5);
// }

// 部分 move
// 当解构一个变量时，可以同时使用 move 和引用模式绑定的方式。当这么做时，
// 部分 move 就会发生：变量中一部分的所有权被转移给其它变量，而另一部分我们获取了它的引用。
//
// 在这种情况下，原变量将无法再被使用，但是它没有转移所有权的那一部分依然可以使用，也就是之前被引用的那部分。
//
// 示例

//todo:
// fn main() {
//     #[derive(Debug)]
//     struct Person {
//         name: String,
//         age: Box<u8>,
//     }
//     let person = Person {
//         name: String::from("Alice"),
//         age: Box::new(20),
//     };
//     // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
//     // 但是，这里 `age` 变量确是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age
//     let Person {
//         name,
//         ref age
//     } = person;
//     println!("The person's age is {}", age);
//     println!("The person's name is {}", name);
//     // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
//     //println!("The person struct is {:?}", person);
//     // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
//     println!("The person's age from person struct is {}", person.age);
// }


// fn main() {
//     let t = (String::from("hello"), String::from("world"));
//
//     let _s = t.0;
//
//     // 仅修改下面这行代码，且不要使用 `_s`
//     println!("{:?}", t.1);
// }

// fn main() {
//     let t = (String::from("hello"), String::from("world"));
//
//     // 填空，不要修改其它代码
//     let (s1, s2) = &t;
//
//     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
// }


// 引用和借用
// 引用
// fn main() {
//     let x = 5;
//     // 填写空白处
//     let p = &x;
//     println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
// }


// fn main() {
//     let x = 5;
//     let y = &x;
//     // 只能修改以下行
//     assert_eq!(5, *y);
// }

// 修复错误
// fn main() {
//     let s = String::from("hello, ");
//
//     borrow_object(&s);
// }
//
// fn borrow_object(s: &String) {
//     println!("{}", s)
// }

// 修复错误
// fn main() {
//     let mut s = String::from("hello, ");
//
//     push_str(&mut s);
// }
//
// fn push_str(s: &mut String) {
//     s.push_str("world")
// }

// fn main() {
//     let mut s = String::from("hello, ");
//
//     // 填写空白处，让代码工作
//     let p = &mut s;
//
//     p.push_str("world");
//     println!("{}", p);
// }

// fn main() {
//     let c = '中';
//
//     let r1 = &c;
//     // 填写空白处，但是不要修改其它行的代码
//     let ref r2 = c;
//
//     assert_eq!(*r1, *r2);
//
//     // 判断两个内存地址的字符串是否相等
//     assert_eq!(get_addr(r1), get_addr(r2));
// }
//
// // 获取传入引用的内存地址的字符串形式
// fn get_addr(r: &char) -> String {
//     format!("{:p}", r)
// }

//借用规则
// 移除代码某个部分，让它工作
// 你不能移除整行的代码！
// fn main() {
//     let s = String::from("hello");
//     let r1 = &s;
//     let r2 = &s;
//     println!("{}, {}", r1, r2);
// }


// 可变性
// 🌟 错误: 从不可变对象借用可变
// fn main() {
//     // 通过修改下面一行代码来修复错误
//     let mut s = String::from("hello, ");
//     borrow_object(&mut s);
// }
//
// fn borrow_object(s: &mut String) {
//     println!("{}", s);
// }

//Ok: 从可变对象借用不可变
// 下面的代码没有任何错误
// fn main() {
//     let mut s = String::from("hello, ");
//     borrow_object(&s);
//     s.push_str("world");
//     println!("{},", s);
// }
//
// fn borrow_object(_s: &String) {}

// NLL
// 🌟🌟


//todo:
// 注释掉一行代码让它工作
// fn main() {
//     let mut s = String::from("hello, ");
//     let r1 = &mut s;
//     r1.push_str("world");
//     let r2 = &mut s;
//     r2.push_str("!");
//     //println!("{}",r1);
// }


// fn main() {
//     let mut s = String::from("hello, ");
//     let r1 = &mut s;
//     let r2 = &mut s;
//     // 在下面增加一行代码人为制造编译错误：cannot borrow `s` as mutable more than once at a time
//     // 你不能同时使用 r1 和 r2
//     r1.push_str("world!");
// }

// 字符串
// 字符串字面量的类型是 &str， 例如 let s: &str = "hello, world" 中的 "hello, world" 的类型就是 &str。
//
// str 和 &str
// 🌟 正常情况下我们无法使用 str 类型，但是可以使用 &str 来替代
// 修复错误，不要新增代码行
// fn main() {
//     let s: &str = "hello, world";
// }

//🌟🌟 如果要使用 str 类型，只能配合 Box。 & 可以用来将 Box<str> 转换为 &str 类型
// 使用至少两种方法来修复错误
// fn main() {
//     let s: Box<str> = "hello,world".into();
//     greetings(&s);
// }
//
// fn greetings(s: &str) {
//     println!("{}", s);
// }

// String
// String 是定义在标准库中的类型，分配在堆上，可以动态的增长。它的底层存储是动态字节数组的方式( Vec<u8> )，
// 但是与字节数组不同，String 是 UTF-8 编码。
// fn main() {
//     let mut s = String::new();
//     s.push_str("hello,world");
//     s.push_str("!");
//     assert_eq!(s, "hello,world!");
// }

// 修复所有错误，并且不要新增代码行
// fn main() {
//     let mut s = String::from("hello");
//     s.push(',');
//     s.push_str("world");
//     s += "!";
//     println!("{}", s);
// }

//🌟🌟 我们可以用 replace 方法来替换指定的子字符串
// fn main() {
//     let s = String::from("i like dogs");
//     let s1 = s.replace("dogs", "cats");
//     assert_eq!(s1, "i like cats");
// }

//🌟🌟 你只能将 String 跟 &str 类型进行拼接，并且 String 的所有权在此过程中会被 move
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = String::from(",world");
//     let s3 = s1.clone() + &s2;
//     assert_eq!(s3, "hello,world");
//     println!("{}", s1);
// }

// &str 和 String
// 与 str 的很少使用相比，&str 和 String 类型却非常常用，因此也非常重要。
// 我们可以使用两种方法将 &str 转换成 String 类型
// fn main() {
//     let s = "hello,world";
//     greetings(s.to_string());
// }
//
// fn greetings(s: String) {
//     println!("{}", s);
// }

// 我们可以使用 String::from 或 to_string 将 &str 转换成 String 类型
// fn main() {
//     let s = "hello,world".to_string();
//     let s1: &str = &s;
// }


//字符串转义
//todo:
// fn main() {
//     // 你可以使用转义的方式来输出想要的字符，这里我们使用十六进制的值，例如 \x73 会被转义成小写字母 's'
//     // 填空以输出 "I'm writing Rust"
//     let byte_escape = "I'm writing Ru\x73__!";
//     println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
//     // 也可以使用 Unicode 形式的转义字符
//     let unicode_codepoint = "\u{211D}";
//     let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
//     println!("Unicode character {} (U+211D) is called {}",
//              unicode_codepoint, character_name );
//     // 还能使用 \ 来连接多行字符串
//     let long_string = "String literals
//                         can span multiple lines.
//                         The linebreak and indentation here \
//                          can be escaped too!";
//     println!("{}", long_string);
// }


//有时候需要转义的字符很多，我们会希望使用更方便的方式来书写字符串: raw string.
//todo:
// fn main() {
//     let raw_str = "Escapes don't work here: \x3F \u{211D}";
//     // 修改以下代码行，让它工作
//     assert_eq!(raw_str, "Escapes don't work here: ? ℝ");
//     // 如果你希望在字符串中使用双引号，可以使用以下形式
//     let quotes = r#"And then I said: "There is no escape!""#;
//     println!("{}", quotes);
//     // 如果希望在字符串中使用 # 号，可以如下使用：
//     let delimiter = r###"A string with "# in it. And even "##!"###;
//     println!("{}", delimiter);
//     // 填空
//     let long_delimiter = r###"Hello, "##""###;
//     assert_eq!(long_delimiter, "Hello, \"##\"")
// }

// 字节字符串
// 想要一个非 UTF-8 形式的字符串吗(我们之前的 str, &str, String 都是 UTF-8 字符串) ?
// 可以试试字节字符串或者说字节数组:
//todo:
// use std::str;
// fn main() {
//     // 注意，这并不是 `&str` 类型了！
//     let bytestring: &[u8; 21] = b"this is a byte string";
//     // 字节数组没有实现 `Display` 特征，因此只能使用 `Debug` 的方式去打印
//     println!("A byte string: {:?}", bytestring);
//     // 字节数组也可以使用转义
//     let escaped = b"\x52\x75\x73\x74 as bytes";
//     // ...但是不支持 unicode 转义
//     // let escaped = b"\u{211D} is not allowed";
//     println!("Some escaped bytes: {:?}", escaped);
//     // raw string
//     let raw_bytestring = br"\u{211D} is not escaped here";
//     println!("{:?}", raw_bytestring);
//     // 将字节数组转成 `str` 类型可能会失败
//     if let Ok(my_str) = str::from_utf8(raw_bytestring) {
//         println!("And the same as text: '{}'", my_str);
//     }
//     let _quotes = br#"You can also use "fancier" formatting, \
//                     like with normal raw strings"#;
//     // 字节数组可以不是 UTF-8 格式
//     let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS
//     // 但是它们未必能转换成 `str` 类型
//     match str::from_utf8(shift_jis) {
//         Ok(my_str) => println!("Conversion successful: '{}'", my_str),
//         Err(e) => println!("Conversion failed: {:?}", e),
//     };
// }

// 字符串索引string index
// 🌟🌟 你无法通过索引的方式去访问字符串中的某个字符，但是可以使用切片的方式 &s1[start..end] ，
// 但是start 和 end 必须准确落在字符的边界处.
// fn main() {
//     let s1 = String::from("hi,中国");
//     let h = &s1[0..1];
//     assert_eq!(h, "h");
//     let h1 = &s1[3..6];
//     assert_eq!(h1, "中");
// }

//操作 UTF-8 字符串
// fn main() {
//     for c in "你好，世界!".chars() {
//         print!("{}", c)
//     }
// }


//todo:
// use utf8_slice;
// fn main() {
//     let s = "The 🚀 goes to the  🌑!";
//     let rocket = utf8_slice::slice(s, 4, 5);
// }


// 数组
// 数组的类型是 [T; Lengh], 就如你所看到的，数组的长度是类型签名的一部分，
// 因此数组的长度必须在编译期就已知，例如你不能使用以下方式来声明一个数组:
//
// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     assert_eq!(arr.len(), 5);
// }

// fn main() {
//     // 很多时候，我们可以忽略数组的部分类型，也可以忽略全部类型，让编译器帮助我们推导
//     let arr0 = [1, 2, 3];
//     let arr: [char; 3] = ['a', 'b', 'c'];
//     // 填空
//     // 数组分配在栈上， `std::mem::size_of_val` 函数会返回整个数组占用的内存空间
//     // 数组中的每个 char 元素占用 4 字节的内存空间，因为在 Rust 中， char 是 Unicode 字符
//     assert_eq!(std::mem::size_of_val(&arr), arr0.len()*4)
// }


//数组中的所有元素可以一起初始化为同一个值
// fn main() {
//     let list: [i32; 100] = [1; 100];
//     assert_eq!(list[0], 1);
//     assert_eq!(list.len(), 100);
// }

//数组中的所有元素必须是同一类型
// fn main() {
//     let _arr = [1, 2, 3];
// }


// 数组的下标索引从 0 开始.
// fn main() {
//     let arr = ['a', 'b', 'c'];
//     let ele = arr[1];
//     assert_eq!(ele, 'b');
// }

//越界索引会导致代码的 panic.
// fn main() {
//     let names = [String::from("Sunfei"), "Sunface".to_string()];
//     // `get` 返回 `Option<T>` 类型，因此它的使用非常安全
//     let name0 = names.get(0).unwrap();
//     println!("{}", name0);
//     // 但是下标索引就存在越界的风险了
//     let _name1 = &names[1];
// }

// 切片( Slice )
// 切片跟数组相似，但是切片的长度无法在编译期得知，因此你无法直接使用切片类型。
// 🌟🌟 这里, [i32] 和 str 都是切片类型，但是直接使用它们会造成编译错误，
// 如下代码所示。为了解决，你需要使用切片的引用： &[i32], &str.
// 修复代码中的错误，不要新增代码行!
// fn main() {
//     let arr = [1, 2, 3];
//     let s1: &[i32] = &arr[0..2];
//     let s2: &str = "hello,world";
// }

// fn main() {
//     let arr: [char; 3] = ['中', '国', '人'];
//     let slice = &arr[..2];
//     println!("{:?}", slice);
//     // 修改数字 `6` 让代码工作
//     // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过：
//     // 因为'中'和'国'是 UTF-8 字符，它们每个占用 3 个字节，2 个字符就是 6 个字节
//     assert_eq!(std::mem::size_of_val(&slice), 16);
// }

// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     // 填空让代码工作起来
//     let slice: &[i32] = &arr[1..4];
//     assert_eq!(slice, &[2, 3, 4]);
// }

//字符串切片
// fn main() {
//     let s = String::from("hello");
//     let slice1 = &s[0..2];
//     let slice2 = &s[..2];
//     assert_eq!(slice1, slice2);
// }

// fn main() {
//     let s = "你好，世界";
//     let slice = &s[0..3];
//     assert_eq!(slice, "你");
// }

// fn main() {
//     let mut s = String::from("hello,world");
//     let word = first_word(&s);
//     //s.clear();
//     println!("the first word is: {}", word);
//     s.clear();
// }
//
// fn first_word(s: &str) -> &str {
//     &s[..1]
// }

// 元组( Tuple )
// 🌟 元组中的元素可以是不同的类型。元组的类型签名是 (T1, T2, ...), 这里 T1, T2 是相对应的元组成员的类型.
// fn main() {
//     let _t0: (u8, i16) = (0, -1);
//     let _t1: (u8, (i16, u32)) = (0, (-1, 1));
//     let _t: (u8, u16, i16, &str, String) = (1u8, 2u16, 3i16, "hello", String::from(",world"));
// }

//🌟 可以使用索引来获取元组的成员
// fn main() {
//     let t = ("i", "am", "sunface");
//     assert_eq!(t.2, "sunface");
// }

// 过长的元组无法被打印输出
// fn main() {
//     let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
//     println!("too long tuple:{:?}", too_long_tuple);
// }

//使用模式匹配来解构元组
// fn main() {
//     let tup = (1, 6.4, "hello");
//     let (x, y, z) = tup;
//     assert_eq!(x, 1);
//     assert_eq!(y, 6.4);
//     assert_eq!(z, "hello");
// }

//解构式赋值
// fn main() {
//     //let (x, y, z)=();
//     let (y, z, x) = (1, 2, 3);
//     assert_eq!(x, 3);
//     assert_eq!(y, 1);
//     assert_eq!(z, 2);
// }

//元组可以用于函数的参数和返回值
// fn main() {
//     let (x, y) = sum_multiply((5, 6));
//     assert_eq!(x, 11);
//     assert_eq!(y, 30);
// }
//
// fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
//     (nums.0 + nums.1, nums.0 * nums.1)
// }

// 结构体
// 三种类型的结构体
// 🌟 对于结构体，我们必须为其中的每一个字段都指定具体的值
// fn main() {
//     let age = 30;
//     let p = Person {
//         name: String::from("sunface"),
//         age,
//         hobby: "coding".to_string(),
//     };
// }
//
// struct Person {
//     name: String,
//     age: u8,
//     hobby: String,
// }

//🌟 单元结构体没有任何字段。
// struct Unit;
//
// trait SomeTrait {}
//
// impl SomeTrait for Unit {}
//
// fn main() {
//     let u = Unit;
//     do_domething_with_unit(u);
// }
//
// fn do_domething_with_unit(u: Unit) {}


// 🌟🌟🌟 元组结构体看起来跟元组很像，但是它拥有一个结构体的名称，
// 该名称可以赋予它一定的意义。由于它并不关心内部数据到底是什么名称，因此此时元组结构体就非常适合。
// struct Color(i32, i32, i32);
//
// struct Point(i32, i32, i32);
//
// fn main() {
//     let v = Point(0, 127, 255);
//     check_color(v);
// }
//
// fn check_color(p: Point) {
//     let (x, _, _) = p;
//     assert_eq!(x, 0);
//     assert_eq!(p.1, 127);
//     assert_eq!(p.2, 255);
// }

// 🌟🌟🌟 元组结构体看起来跟元组很像，但是它拥有一个结构体的名称，该名称可以赋予它一定的意义。
// 由于它并不关心内部数据到底是什么名称，因此此时元组结构体就非常适合。
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// fn main() {
//     let v = Point(0, 127, 255);
//     check_color(v);
// }
//
// fn check_color(p: Point) {
//     let Point(x, _, _) = p;
//     assert_eq!(x, 0);
//     assert_eq!(p.1, 127);
//     assert_eq!(p.2, 255);
// }

// 结构体上的一些操作
// 🌟 你可以在实例化一个结构体时将它整体标记为可变的，但是 Rust 不允许我们将结构体的某个字段专门指定为可变的.
// struct Person {
//     name: String,
//     age: u8,
// }
//
// fn main() {
//     let age = 18;
//     let mut p = Person {
//         name: String::from("sunface"),
//         age,
//     };
//     println!("{},{}", p.name, p.age);
//     p.age = 30;
//     p.name = String::from("sunfei");
//     println!("{},{}", p.name, p.age);
// }

//🌟 使用结构体字段初始化缩略语法可以减少一些重复代码
// struct Person {
//     name: String,
//     age: u8,
// }
//
// fn main() {
//     let person = build_person(String::from("zhangsan"), 18);
//     println!("{},{}", person.age, person.name);
// }
//
// fn build_person(name: String, age: u8) -> Person {
//     Person {
//         age,
//         name
//     }
// }

//🌟 你可以使用结构体更新语法基于一个结构体实例来构造另一个
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
//
// fn main() {
//     let u1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("sunface"),
//         active: true,
//         sign_in_count: 1,
//     };
//     println!("{},{},{},{}", u1.email, u1.username, u1.active, u1.sign_in_count);
//     let u2 = set_email(u1);
//     println!("{},{},{},{}", u2.email, u2.username, u2.active, u2.sign_in_count);
// }
//
// fn set_email(u: User) -> User {
//     User {
//         email: String::from("contact@im.dev"),
//         ..u
//     }
// }

// 打印结构体
// 🌟🌟 我们可以使用 #[derive(Debug)] 让结构体变成可打印的.
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };
//     dbg!(&rect1);
//     println!("{:?}", rect1);
// }

// 结构体的所有权
// 当解构一个变量时，可以同时使用 move 和引用模式绑定的方式。当这么做时，
// 部分 move 就会发生：变量中一部分的所有权被转移给其它变量，而另一部分我们获取了它的引用。
// 在这种情况下，原变量将无法再被使用，但是它没有转移所有权的那一部分依然可以使用，也就是之前被引用的那部分。
// fn main() {
//     struct Person {
//         name: String,
//         age: Box<u8>,
//     }
//     let person = Person {
//         name: String::from("alice"),
//         age: Box::new(20),
//     };
//     let Person {
//         name, ref age
//     } = person;
//     println!("the person's age is {}", age);
//     println!("the person's name is {}", name);
//     println!("the person's age from person struct is {}", person.age);
// }

// struct File {
//     name: String,
//     data: String,
// }
//
// fn main() {
//     let f = File {
//         name: String::from("readme.md"),
//         data: "Rust By Practice".to_string(),
//     };
//     let _name = f.name;
//     println!("{}", f.data);
// }

// 枚举 Enum
// 🌟🌟 在创建枚举时，你可以使用显式的整数设定枚举成员的值。
// enum Number {
//     Zero,
//     One,
//     Two,
// }
//
// enum Number1 {
//     Zero = 0,
//     One,
//     Two,
// }
//
// enum Number2 {
//     Zero = 0,
//     One = 1,
//     Two = 2,
// }
//
// fn main() {
//     assert_eq!(Number::One as u8, Number1::One as u8);
//     assert_eq!(Number1::One as u8, Number2::One as u8);
// }

//🌟 枚举成员可以持有各种类型的值
// enum Message {
//     Quit,
//     Move {
//         x: i32,
//         y: i32,
//     },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// fn main() {
//     let msg1=Message::Move {x:1,y:2};
//     let msg2=Message::Write(String::from("hello,world"));
// }

//todo:
// 🌟🌟 枚举成员中的值可以使用模式匹配来获取
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn main() {
//     let msg = Message::Move { x: 1, y: 1 };
//     if let Message::Move { x: a, y: b } = msg {
//         assert_eq!(a, b);
//     } else {
//         panic!("不执行");
//     }
// }


//🌟🌟 使用枚举对类型进行同一化
//todo:
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move {
//         x: i32,
//         y: i32,
//     },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn main() {
//     let msgs: [Message; 3] = [
//         Message::Quit,
//         Message::Move { x: 1, y: 3 },
//         Message::ChangeColor(255, 255, 0)
//     ];
//     for msg in msgs {
//         show_message(msg)
//     }
// }
// fn show_message(msg: Message) {
//     println!("{:?}", msg);
// }


//🌟🌟 Rust 中没有 null，我们通过 Option<T> 枚举来处理值为空的情况
//todo:
// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     if let Some(n) = six {
//         println!("{}", n);
//         return
//     }
//     panic!("不要执行");
// }
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

//TODO:
//🌟🌟🌟🌟 使用枚举来实现链表.
// use crate::List::*;
//
// enum List {
//     Cons(u32, Box<List>),
//     Nil,
// }
//
// impl List {
//     fn new() -> List {
//         Nil
//     }
//     fn prepend(self, elem: u32) -> List {
//         Cons(elem, Box::new(self))
//     }
//     fn len(&self) -> u32 {
//         match *self {
//             Cons(_, ref tail) => 1 + tail.len(),
//             Nil => 0
//         }
//     }
//     fn stringify(&self) -> String {
//         match *self {
//             Cons(head, ref tail) => {
//                 format!("{}, {}", head, tail.stringify())
//             }
//             Nil => {
//                 format!("Nil")
//             }
//         }
//     }
// }
//
// fn main() {
//     let mut list = List::new();
//     list = list.prepend(1);
//     list = list.prepend(2);
//     list = list.prepend(3);
//     println!("链表的长度是：{}", list.len());
//     println!("{}", list.stringify());
// }

// 流程控制
// if/else
// fn main() {
//     let n = 5;
//     if n < 0 {
//         println!("{} is negative", n);
//     } else if n > 0 {
//         println!("{} is positive", n);
//     } else {
//         println!("{} is zero", n);
//     }
// }

// fn main() {
//     let n = 5;
//     let big_n = if n < 10 && n > -10 {
//         println!("数字太小，先增加10倍再说");
//         10 * n
//     } else {
//         println!("数字太大，我们得让他减半");
//         n / 2
//     };
//     println!("{}->{}", n, big_n);
// }

// fn main() {
//     for n in 1..100 {
//         if n == 100 {
//             panic!("never let this run")
//         }
//     }
// }

// fn main() {
//     let names = [String::from("liming"), String::from("hanameimei")];
//     for name in &names {}
//     println!("{:?}", names);
//     let numbers = [1, 2, 3];
//     for n in numbers {}
//     println!("{:?}", numbers);
// }

// fn main() {
//     let a = [4, 3, 2, 1];
//     for (i, v) in a.iter().enumerate() {
//         println!("第{}个元素是{}", i + 1, v);
//     }
// }

// while
// 🌟🌟 当条件为 true 时，while 将一直循环
// fn main() {
//     let mut n = 1;
//     while n < 10 {
//         if n % 15 == 0 {
//             println!("fizzbuzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else if n % 5 == 0 {
//             println!("{}", n);
//         }
//         n += 1;
//     }
//     println!("n的值是{}", n);
// }

// continue and break
// 🌟 使用 break 可以跳出循环
// fn main() {
//     let mut n = 0;
//     for i in 0..=100 {
//         if n == 66 {
//             break;
//         }
//         n += 1;
//         println!("{}",i);
//     }
//     assert_eq!(n, 66);
// }


//🌟🌟 continue 会结束当次循环并立即开始下一次循环
// fn main() {
//     let mut n = 0;
//     for _i1 in 0..=100 {
//         if n != 66 {
//             n += 1;
//             continue;
//         }
//         break
//     }
//     assert_eq!(n, 66);
// }

// loop
// 🌟🌟 loop 一般都需要配合 break 或 continue 一起使用。
// fn main() {
//     let mut count = 0u32;
//     println!("let's count until infinity!");
//     loop {
//         count += 1;
//         if count == 3 {
//             println!("three");
//             continue;
//         }
//         println!("{}", count);
//         if count == 5 {
//             println!("ok,that's enough");
//             break;
//         }
//     }
//     assert_eq!(count, 5);
// }

//🌟🌟 loop 是一个表达式，因此我们可以配合 break 来返回一个值
// fn main() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter*2;
//         }
//     };
//     assert_eq!(result, 20);
// }

// 🌟🌟🌟 当有多层循环时，你可以使用 continue 或 break 来控制外层的循环。
// 要实现这一点，外部的循环必须拥有一个标签 'label, 然后在 break 或 continue 时指定该标签
//todo:
// fn main() {
//     let mut count = 0;
//     'outer: loop {
//         'inner1: loop {
//             if count >= 20 {
//                 break 'inner1;
//             }
//             count += 2;
//         }
//         count += 5;
//         'inner2: loop {
//             if count >= 30 {
//                 break 'outer;
//             }
//             continue 'outer;
//         }
//     }
//     assert_eq!(count, 30);
// }

// match, matches! 和 if let
// match
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }
//
// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::South | Direction::North => {
//             println!("South or North");
//         }
//         _ => println!("West"),
//     };
// }

//🌟🌟 match 是一个表达式，因此可以用在赋值语句中
// fn main() {
//     let boolean = true;
//     let binary = match boolean {
//         true => 1,
//         false => 0
//     };
//     assert_eq!(binary, 1);
// }

//🌟🌟 使用 match 匹配出枚举成员持有的值
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// fn main() {
//     let msgs = [
//         Message::Quit,
//         Message::Move { x: 1, y: 3 },
//         Message::ChangeColor(255, 255, 0)
//     ];
//     for msg in msgs {
//         show_message(msg);
//     }
// }
//
// fn show_message(msg: Message) {
//     match msg {
//         Message::Move { x: a, y: b } => {
//             assert_eq!(a, 1);
//             assert_eq!(b, 3);
//             println!("参数是{}和{}", a, b);
//         }
//         Message::ChangeColor(r, g, b) => {
//             assert_eq!(g, 255);
//             assert_eq!(b, 0);
//             println!("参数是{}和{}和{}", r, g, b);
//         }
//         __ => println!("no data in these variants")
//     }
// }

// matches!
// matches! 看起来像 match, 但是它可以做一些特别的事情
// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];
//     for ab in alphabets {
//         assert!(matches!(ab,'a'..='z' | 'A'..='Z' | '0' | '9'))
//     }
// }


// enum MyEnum {
//     Foo,
//     Bar,
// }
//
// fn main() {
//     let mut count = 0;
//     let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Bar];
//     for e in v {
//         if matches!(e,MyEnum::Foo) {
//             count += 1;
//         }
//     }
//     assert_eq!(count, 2);
// }

// if let
// 在有些时候, 使用 match 匹配枚举有些太重了，此时 if let 就非常适合.
//todo:
// fn main() {
//     let o = Some(7);
//     if let Some(i) = o {
//         println!("this is a raally long string and '{:?}'", i);
//     }
// }

// enum Foo {
//     Bar(u8)
// }
//
// fn main() {
//     let a = Foo::Bar(1);
//     if let Foo::Bar(i) = a {
//         println!("{}", i);
//     }
// }


// enum Foo {
//     Bar,
//     Baz,
//     Qux(u32),
// }
//
// fn main() {
//     let a = Foo::Qux(10);
//     match a {
//         Foo::Bar => println!("match foo::bar"),
//         Foo::Baz => println!("match foo::baz"),
//         _ => println!("match others"),
//     }
// }

//变量遮蔽( Shadowing )
//todo:
// fn main() {
//     let age = Some(30);
//     if let Some(age) = age {
//         assert_eq!(age, 30);
//     }
//     match age {
//         Some(age) => println!("age是一个新变量，它的值是{}", age),
//         _ => ()
//     }
// }


// 模式
// 🌟🌟 使用 | 可以匹配多个值, 而使用 ..= 可以匹配一个闭区间的数值序列
// fn main() {}
//
// fn match_number(n: i32) {
//     match n {
//         1 => println!("One"),
//         2 | 3 | 4 | 5 => println!("match 2 -> 5"),
//         6..=10 => {
//             println!("match 6 -> 10")
//         }
//         _ => {
//             println!("match 11-> +infinte")
//         }
//     }
// }

//🌟🌟🌟 @ 操作符可以让我们将一个与模式相匹配的值绑定到新的变量上
// struct Point {
//     x: i32,
//     y: i32,
// }
//
// fn main() {
//     let p = Point { x: 2, y: 20 };
//     match p {
//         Point {
//             x, y: 0
//         } => println!("On the x axis at {}", x),
//         Point {
//             x: 0..=5, y: y @ (10 | 20 | 30)
//         } => println!("On the y axis at {}", y),
//         Point {
//             x, y
//         } => println!("On neither axis:{},{}", x, y),
//     }
// }

// enum Message {
//     Hello {
//         id: i32
//     },
// }
//
// fn main() {
//     let msg = Message::Hello { id: 5 };
//     match msg {
//         Message::Hello {
//             id: id @ 3..=7,
//         } => println!("id值的范围在3，7之间：{}",id),
//         Message::Hello { id: newid @ ( 10 | 11 | 12) } => {
//             println!("id值的范围在10，12之间：{}",newid)
//         }
//         Message::Hello { id } => println!("found some other id:{}", id),
//     }
// }






















