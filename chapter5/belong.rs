/**
 * 控制资源的释放
 * 出街所有权
 * 转移所有权
 */

// #[derive(Debug)]
// struct A{
//     a:i32,
//     b:i32,
// }
//编译报错a的所有权发生转移
// fn main(){
//     let a =A{a:1,b:2};
//     let b = a;
//     println!("{:?}",a);
// }

#[derive(Debug,Copy,Clone)]

struct A{
    a:i32,
    b:i32,
}
