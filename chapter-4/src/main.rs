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
    println!("{}", y);
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);
}
