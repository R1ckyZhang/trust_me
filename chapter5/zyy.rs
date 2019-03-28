//作用域

fn main(){
    //花括号
    //math匹配
    let v = [1,2,3];
    foo(v);
    println!("{:?}",v);
}
fn foo(mut v:[i32;3]) ->[i32;3]{
    v[0] = 3;
    assert_eq!([3,2,3], v);
    println!("{:?}",v);
    v
}