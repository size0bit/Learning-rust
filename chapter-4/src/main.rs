fn main() {
    /*
    什么是所有权：它使用包含特定规则的所有权系统来管理内存，
    这套规则允许编译器在编译过程中执行检查工作，
    而不会产生任何的运行时开销
    */

    /*
    所有权规则：
    Rust中的每一个值都有一个对应变量作为它的所有者
    在同一时间内，值有且仅有一个所有者
    当所有者离开自己的作用域，它持有的值就会被释放掉
    */

    /*
    变量作用域：
    s在进入作用域后变得有效
    它会保持自己的有效性直到自己离开作用域为止
    */
    let _s = "hello";

    /*
    String类型：
    */
    let _s = String::from("hello");
    let mut s = String::from("hello");
    s.push_str(",world");//函数向String空间尾部添加一个字面量
    println!("{}", s);

    /*
    内存与分配：
    我们使用的内存是由操作系统在运行时动态分配出来的
    当使用完String时，我们需要通过某种方式将这些内存归还给操作系统
    */
    let x = 5;
    let y = x;
    println!("y={}", y);
    println!("x={}", x);
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}，world", s2);
    //println!("{}",s1);
    /*
    所有权与函数
    */
    //变量s进入作用域
    let s = String::from("hello");
    //s的值被移动进入了函数//所以它从这里开始不再有效
    takes_ownership(s);
    //let a = s;//s不再可以使用
    //变量x进入作用域
    let x = 5;
    //变量x被传递进入函数
    makes_copy(x);
    //但是由于i32是copy的，所以我们依然可以在这之后使用x
    let _a = x;//s依旧可以使用

    /*
    返回值与作用域：函数在返回值的过程中也会发生所有权的转移
    */
    //gives_ownership将它的返回值移动之s1中
    let s1 = gives_ownership();
    //s2进入作用域
    let s2 = String::from("hello");
    //s2被移动进入函数
    let s3 = takes_and_gives_back(s2);
    //takes_and_gives_back中这个函数的返回值又被移动到了变量s3上
    //s3离开作用域会被销毁。由于s2已经移动了,
    //所以它不会在离开作用域时发生任何事情。s1最后离开作用域并被销毁。
    //let a = s1;//无效引用
    //let a = s2;
    println!("{}s1", s1);
    let a = s3;//s3未转移多有权
    println!("{},", a)
    /*
    变量所有权的转移总是遵循相同的模式：将一个值赋值给另一个变量时就会转移所有权。
    当一个持有堆数据的变量离开作用域时，它的数据就会被drop清理回收，
    除非这些数据的所有权移动到了另一个变量上
    */
}

//some_string进入作用域
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}//some_string在这里离开作用域，drop函数被自动调用


//some_integer进入作用域
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}//some_integer在这里离开了作用域，没有什么特别的事情发生

//gives_ownership会将它的返回值移动至调用它的函数内
fn gives_ownership() -> String {
    //some_string进入作用域
    let some_string = String::from("hello");
    //some_string作为返回值移动之调用函数
    some_string
}

//takes_and_gives_back将取得一个String的所有权并将它作为结果返回
fn takes_and_gives_back(a_string: String) -> String {//a_string进入作用域
    a_string//a_string作为返回值移动至调用函数
}
