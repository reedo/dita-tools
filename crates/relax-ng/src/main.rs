use std::fs::read_to_string;
use std::path::PathBuf;

use quick_xml::de::from_str;

use relax_ng::Pattern;

/// TODO: Document this!
fn main() {
    let xml = read_xml().unwrap();
    let object: Pattern = from_str(&xml).unwrap();
    println!("{:#?}", object);
}

/// TODO: Document this!
fn read_xml() -> Result<String, std::io::Error> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("res");
    path.push("schema.rng");

    println!("File path: {}", path.display());

    read_to_string(path)
}
