fn main() {
    //变量与可变性
    //let a = 5;不能对不可变变量进行二次赋值
    let mut a = 5;
    println!("a的值是{}", a);
    a = 6;
    println!("a的值是{}", a);

    //变量与常量之间的不同
    /*
    不能用mut关键字来修饰一个常量
    常量不仅是默认不可变的，它还总是不可变的
    常量可以被声明在任何作用域
    只能将常量绑定到一个常量表达式上，而无法将一个函数的返回值*/
    const _MAX_POINTS: u32 = 100_000;

    //隐藏
    //我们可以重复使用let关键字并配以相同的名称来不断地隐藏变量
    let b = 5;
    //let b += 1;这是语句，不是表达式
    let b = b + 1;
    let b = b * 2;
    println!("b的值是{}", b);

    /*
    隐藏机制与mut的另一个区别在于：
    由于重复使用let关键字会创建出新的变量，
    所以我们可以在复用变量名称的同时改变它的类型*/
    let spaces = "   ";
    println!("这是一个空字符串{}", spaces);
    let spaces = spaces.len();
    println!("{}", spaces);

    //数据类型
    let _guess: u32 = "42".parse().expect("不是数字类型");

    //标量类型
    /*
    Rust中内建了4种基础的标量类型：整数、浮点数、布尔值及字符
    */
    let _c = 2.0;//双精度浮点
    let _c1: f32 = 3.0;//单精度浮点

    let _d = true;
    let _d1: bool = false;//附带显示类型标注的语句

    //Rust中的char类型占4字节，是一个Unicode标量值
    let _e = 'z';
    let _heart_eyed_cat = '🐱';

    //复合类型:Rust提供了两种内置的基础复合类型：元组 （tuple）和数组 （array）
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup的值是{:#?}", tup);

    //解构
    let (x, y, z) = tup;
    println!("x的值是{},y的值是{},z的值是{}", x, y, z);

    //通过索引并使用点号（.）来访问元组中的值
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred的值是{},six_point_four的值是{},one的值是{}", five_hundred, six_point_four, one);

    //数组
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("{:?}\n{:?}", a, months);

    //访问数组元素
    let first = a[0];
    let second = a[1];
    println!("{},{}", first, second);
    //let index = 10;数组越界
    let index = 4;
    let element = a[index];
    println!("{}", element);

    //函数
    another_function();

    //函数参数
    another_function1(5);
}

fn another_function() {
    println!("Another function");
}

fn another_function1(x: i32) {
    println!("x的值是{}", x);
}
