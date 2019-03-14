enum Number{
    Zero,
    One,
    Two,
}
fn main(){
    let a  = Number::One;
    match a {
        Number::Zero => print!("0"),
        Number::One => print!("1"),
        Number::Two => print!("2"),
    }
}