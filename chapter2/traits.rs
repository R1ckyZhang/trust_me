struct Duck;
struct Pig;
trait Fly{
    fn Fly(&self) -> bool;
}
impl Fly for Duck{
    fn Fly(&self)-> bool{
        return true;
    }
}

impl Fly for Pig{
    fn Fly(&self) -> bool{
        return false;
    }
}


fn fly_static<T:Fly>(s:T)->bool{
    s.Fly()
}

fn fly_dn(s:&Fly)-> bool{
    s.Fly()
}

fn main(){
    let pig = Pig;
    let duck = Duck;

    assert_eq!(fly_static::<Pig>(pig),false );

    assert_eq!(fly_static::<Duck>(duck), true);

    assert_eq!(fly_dn(&Pig),false);
    assert_eq!(fly_dn(&Duck), true);
}