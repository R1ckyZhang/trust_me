/**
 * 智能指针
 *  rust中的值 默认分配到占内存 通过Box<T> 装箱
 */

fn main(){
    #[derive(PartialEq)]
    struct Point{
        x:f64,
        y:f64,
    }
    let box_point = Box::new(Point{x:0.0,y:0.0});
    let un_boxed_point:Point  = *box_point;
    assert_eq!(un_boxed_point,Point{x:0.0,y:0.0} );
}