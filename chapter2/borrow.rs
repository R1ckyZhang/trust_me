fn main(){
    let place1 = "hello";
    let place2 = "hello".to_string();
    /**
     * 这段程序在编译阶段会报错
     * 声明了2个绑定 将place1 赋值给other 将place1内存地址 转给other
     * 同理 后续将place2地址 转移给other
     */
    let other = place1;
    println!("{:?}",other);

    let other  = place2;

    println!("{:?}",place2);
}
