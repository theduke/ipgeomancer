use ipgeom_rpsl::parse_objects_iter;
use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let path = env::args().nth(1).expect("missing path");
    let bytes = fs::read(path)?;
    let data = String::from_utf8_lossy(&bytes);
    for res in parse_objects_iter(&data) {
        match res {
            Ok(obj) => println!("{:?}", obj),
            Err(e) => panic!("parse error: {:?}", e),
        }
    }
    Ok(())
}
