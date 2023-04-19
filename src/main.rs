
fn main(){
    let k: Option<()> = serde_json::from_str("null").unwrap();
    println!("{:?}",k);
    println!("{}",serde_json::to_string(&Some(())).unwrap());
}