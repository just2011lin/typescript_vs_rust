#![allow(dead_code, unused_variables)]

pub fn take_value_ownership() {
    let a = String::from("a");
    // a值的所有权转移给了变量b
    let b = a;
}
