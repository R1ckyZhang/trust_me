use std::fmt::Debug;
pub trait Fly{
    fn fly(&self)->bool;
}

#[derive(Debug)]
struct Duck;
#[derive(Debug)]
struct Pig;

impl Fly for Duck{
    fn fly(&self) ->bool{
        return true;
    }
}

impl Fly for Pig{
    fn fly(&self) ->bool{
        return false;
    }
}


fn fly_static(s:impl Fly+Debug)->bool{
    s.fly();
}

fn can_fly(s:impl Fly+Debug)-> impl Fly{
    if s.fly{
        println!("{:?} can fly",s);
    }else{
        println!("{:?} can not fly",s);
    }
    s
}
fn dyn_can_fly(s:impl Fly+Debug+'static) ->Box<dyn Fly>{
      if s.fly{
        println!("{:?} can fly",s);
    }else{
        println!("{:?} can not fly",s);
    }
    s
}

let pig = Pig;
println!("{:?}",fly_static(pig));
let duck = Duck;
println!("{:?}",fly_static(duck));

let pig = Pig;
let pig = can_fly(pig);
let duck =Duck;
let duck = can_fly(duck);
