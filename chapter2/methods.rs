/**
 * 注意 rust与go 相同的地方 函数都为一等公民
 */

//函数作为参数的情况
pub fn math(op: fn(i32,i32)->i32,a:i32,b:i32) ->i32{
    op(a,b)
}

fn sum(a:i32,b:i32) ->i32{
    a+b
}

fn product(a:i32,b:i32) ->i32{
    a * b
}

fn main(){
    let a = 2;
    let b  = 3;
    assert_eq!(math(sum,a,b), 5);
    assert_eq!(math(product,a,b), 6);
}