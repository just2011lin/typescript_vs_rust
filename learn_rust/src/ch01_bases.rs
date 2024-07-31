// 程序入口从main函数开始
// 不过此处不是入口
// 入口是main.rs中的main函数
pub fn main() {
    println!("hello, world");
}

pub fn main_let() {
    let a = 10;
    // 编译错误
    // a = 20;
    println!("a = {a}");
}

pub fn main_let_ref() {
    let a = [10];
    // 不能对a内部进行赋值
    // a[0] = 20;
    println!("a = {a:?}");
}

pub fn main_let_let() {
    let a = 10;
    println!("a = {a}");
    // 可使用let重新声明a变量
    // 并且改变a的类型
    let a = "a";
    // 新的a变量将会隐藏之前的a变量
    println!("a = {a}");
}

pub fn main_let_mut() {
    // 使用let mut声明可变变量
    let mut a = 10;
    println!("a = {a}");
    // 对a重新赋值
    a = 20;
    println!("a = {a}");
}

pub fn main_i8() {
    let a: i8 = 127;
    let b: i8 = 1;

    println!("a + b = {}", a + b);
}

pub fn main_bool() {
    // 使用bool表示布尔值类型
    let a: bool = false;
    println!("a = {a}");
}

pub fn main_char() {
    // 字符存储空间为四个字节，即32位
    // 可以理解为字符串中的某个字符
    let a = '♥';
    println!("a = {a}");
}
