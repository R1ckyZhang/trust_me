use std::collections::HashMap;
use std::collections::BTreeMap;
fn main(){
    let mut hmap = HashMap::new();
    let mut bmap = BTreeMap::new();
    hmap.insert(3,"c");
    hmap.insert(1,"a");
    hmap.insert(2,"b");
    bmap.insert(1,"a");
    bmap.insert(2,"b");
    print!("{:?}",hmap);
    print!("{:?}",bmap);
}