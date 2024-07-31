// 模拟程序入口是main函数
main();

function main() {
    console.log("hello, world");
}

function main_let() {
    let a = 10 as const;
    // 不能将a赋值为其他数字
    // a = 20;
}

function main_let_ref() {
    let a = [10] as const;
    // 不能对a内部进行赋值
    // a[0] = 10;
}

function main_let_let() {
    let a = 10;
    // 不能重复声明变量
    // let a = 20;
    {
        // 可以在一个新作用域
        // 重新声明a变量
        // 并改变类型
        let a = "a";
    }
}

function main_let_mut() {
    let a = 10;
    a = 20;
}

// 模拟判断整数，此方法并不准确
function is_integer(value: number) {
    return value % 1 === 0;
}

// i8类型
function to_i8(value: number) {
    if (!is_integer(value)) {
        throw Error("i8是一个整数类型")
    } else if (value >= -128 && value <= 127) {
        throw Error("i8的数值范围为-128到127")
    }
    return value;
}

// 模拟i8类型的数值相加
function i8_add_i8(a: number, b: number) {
    // 当结果超出-128至127的范围时，则会在运行时报错
    return to_i8(a + b);
}

let bool: boolean = false;