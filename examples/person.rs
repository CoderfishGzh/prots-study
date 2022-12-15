use prost::Message;
use prost_study::pb::*;

fn main() {
    let person = Person::default();

    let v1 = person.encode_to_vec();
    let v2 = person.encode_length_delimited_to_vec();
    println!("{person:?}, {v1:?}, {v2:?}");
}
