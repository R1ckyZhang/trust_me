trait Page{
    fn set_page(&self,p:i32){
        println!("page default:1");
    }
}
trait PerPage{
    fn set_perpage(&self,num:i32){
        println!("default 10");
    }
}

struct MyPage{page :i32}
impl Page for MyPage{}
impl PerPage for MyPage{}
fn main(){
    let my = MyPage{page:1};
    my.set_page(2);
    my.set_perpage(100);
}