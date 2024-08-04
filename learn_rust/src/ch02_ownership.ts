function take_value_ownership() {
    let a: String | void = String("a");
    // 模拟rust中所有权转移
    let b = a;
    // a值的所有权转移给b后
    // 相当于将a重新设置为null
    a = undefined;
}