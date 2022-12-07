use dicecloud_models::data_models::dnd_model::*;
use std::{fs::File, io::Read};
fn main(){
    let mut file = File::open("raulnor_bogdan.json").expect("failed to open file");
    let mut buf = vec![];
    let _=file.read_to_end(&mut buf);
    let char_str = String::from_utf8_lossy(&buf);
    let character = Character::from_str(&char_str).expect("I really hope it works");

}