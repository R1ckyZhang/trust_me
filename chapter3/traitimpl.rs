/**
 * 抽象类型
 * 在范型中使用trait限定，任意类型的范围根据行为限定到精准可控范围
 * 可以将共同拥有相同行为的类型集合抽象为一个类型 trait对象
 */
#[derive(Debug)]
struct Foo;
trait Bar{
    fn baz(&self);
}

impl Bar for Foo{
    fn baz(&self){println!("{:?}",self)}
}

impl static_dispatch<T>(t: &T) where T:Bar{
    t.baz();
}

fn dynamic_dispath(t:&Bar){
    t.baz();
}
fn main(){
    let foo = Foo;
    static_dispatch(&foo);
    dynamic_dispath(&foo);
}
