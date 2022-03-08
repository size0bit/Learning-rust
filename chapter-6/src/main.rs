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

fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
    // 但是，这里 `age` 变量确是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age
    let Person {
        name,
        ref age
    } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    //println!("The person struct is {:?}", person);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    println!("The person's age from person struct is {}", person.age);
}
