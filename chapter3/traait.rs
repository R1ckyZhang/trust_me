trait Add<RHS,Output>{
    fn add(self,rhs:RHS)->Output;
}
impl Add<i32,i32> for i32{
    fn add(self,rhs:i32)->i32{
        self+rhs
    }
}

impl Add<u32,i32> for u32{
    fn add(self,rhs:u32)->i32{
        (self+rhs) as i32
    }
}

fn main(){
    let (a,b,c,d) =(1i32,2i32,3u32,4u32);
    let y:i32 = a.add(b);
    let x:i32 = c.add(d);
    print!("{:?}",x);
    println!("{:?}",y);
}