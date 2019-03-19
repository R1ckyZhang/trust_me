/**
 * trait 的self类型不能是sized
 * trait 中所有方法必须是对象安全的
 */

trait Bar{
    fn bax(self ,x:u32);
    fn bay(&self);
    fn baz(&mut self);
}
/**
 * 对象安全的方法的规定
 * 1 方法受Self:Sized约束
 * 方法签名 同时满足 不接受范型参数 第一个参数必须为 Self类型或者可以解引用Self类型 Self不能出现在除第一个参数以外的地方
 */