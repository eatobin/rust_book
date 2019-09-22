extern crate rustc_serialize;

use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct Penguin {
    name: String,
    born: i16,
}

fn main() {
    let pengu = Penguin {
        name: "pengu".to_string(),
        born: 1999,
    };
    println!("{}", json::encode(&pengu).unwrap());
    let pingu: Penguin = json::decode(r##"{"name":"pingu","born":2001}"##).unwrap();
    assert_eq!(&pingu.name, "pingu");
    assert_eq!(pingu.born, 2001);
}
