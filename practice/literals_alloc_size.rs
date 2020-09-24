/// 字面量绑定的变量所占用的字节数
fn main() {
    // 带后缀的字面量，其类型在初始化时已经知道了。
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 无后缀的字面量，其类型取决于如何使用它们。
    let i = 1;
    let f = 1.0;

    let string = String::from("hello world, I'm Rust! 返回一个变量所占的字节数。");
    let str_slice = "hello world, I'm Rust! 返回一个变量所占的字节数。。。。";
    
    let vec_list = vec![String::from("hello"),String::from("world")];

    // `size_of_val` 返回一个变量所占的字节数
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    // 编译后大小固定不变的数据类型
    println!("size of `string` in bytes: {}",std::mem::size_of_val(&str_slice));
    // 编译后大小不固定的数据类型
    println!("size of `str_slice` in bytes: {}",std::mem::size_of_val(&string));
    println!("size of `str_slice` in bytes: {}",std::mem::size_of_val(&vec_list));
    
}
// -----output result-----
//  size of `x` in bytes: 1
//  size of `y` in bytes: 4
//  size of `z` in bytes: 4
//  size of `i` in bytes: 4
//  size of `f` in bytes: 8
//  size of `string` in bytes: 16
//  size of `str_slice` in bytes: 24
//  size of `str_slice` in bytes: 24

