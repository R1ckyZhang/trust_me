fn main(){
    let s : &Option<String> = &Some("hello_world".to_string());
    match s {
        Some(s) => print!("s is :{}",s),
        _  =>(),
    }
}

// enum Option{
//     Some(i32),
//     None,
// }