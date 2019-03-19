trait Add<RHS=Self>{
    type Output;
    fn add(self,rhs:RHS)->Self::Output;
}

impl Add<u64> for u32{
    type Output  = u64;
    fn add(self,other:u64)->Self::Output{
        (self as u64)+other
    }
}

fn main(){
    let a = 1u32;
    let b =2u64;
   let c =  a.add(b);
   println!("{:?}",c);
}