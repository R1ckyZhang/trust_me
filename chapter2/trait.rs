/**
 * 范型与trait  
 * trait 是对类型行为的抽象 其组合方式与golang类似
 */

use std::fmt::Debug;

fn match_option<T:Debug>(o:Option<T>){
    match o {
        Some(i) =>print!("{:?}",i),
        None => print!("nothing"),
    }
}

fn main(){
    let a:Option<i32> = Some(3);
    let b:Option<&str> = Some("hello");

    match_option(a);
    match_option(b);
}
