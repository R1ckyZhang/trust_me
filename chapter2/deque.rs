use std::collections::VecDeque;

fn main(){
    let mut buf = VecDeque::new();
    buf.push_front(1);
    buf.push_front(2);
    print!("{:?}",buf);
    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);
    print!("{:?}",buf);
}