use ipgeom_rpsl::{parse_objects_iter, RpslObject};
use serde_json;
use ipgeom_rpsl::parse_objects_read_iter;
use std::env;
use std::fs::File;
use std::io::{self, BufReader};

enum OutputFormat {
    Rpsl,
    Json,
}

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let mut typed = false;
    let mut format = OutputFormat::Rpsl;
    let mut path: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--typed" => typed = true,
            "--json" => format = OutputFormat::Json,
            "--rpsl" => format = OutputFormat::Rpsl,
            _ => {
                if path.is_none() {
                    path = Some(arg);
                } else {
                    eprintln!("Unknown argument: {}", arg);
                }
            }
        }
    }

    let path = path.expect("missing path");

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for res in parse_objects_read_iter(reader) {
        match res {
            Ok(obj) => {
                if typed {
                    let typed_obj = RpslObject::try_from(obj.clone()).unwrap_or(RpslObject::Other(obj.clone()));
                    match format {
                        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&typed_obj).unwrap()),
                        OutputFormat::Rpsl => println!("{}", obj.to_rpsl()),
                    }
                } else {
                    match format {
                        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&obj).unwrap()),
                        OutputFormat::Rpsl => println!("{}", obj.to_rpsl()),
                    }
                }
            }
            Err(e) => panic!("parse error: {:?}", e),
        }
    }
    Ok(())
}
