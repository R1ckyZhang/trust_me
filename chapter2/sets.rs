use std::collections::HashSet;
use std::collections::BTreeSet;
fn main(){
    let mut hbooks = HashSet::new();
    let mut bbooks = BTreeSet::new();
    hbooks.insert("a");
    hbooks.insert("b");
    hbooks.insert("c");
    if !hbooks.contains("d"){
        print!("nooooo {} ",hbooks.len());
    }
    bbooks.insert("a");
    bbooks.insert("b");
    bbooks.insert("c");
    print!("{:?}",bbooks)
}