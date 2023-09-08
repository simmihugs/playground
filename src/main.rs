use std::io::BufReader;
use std::{fs::File, str::FromStr};

use xml::{
    name::OwnedName,
    reader::{EventReader, XmlEvent},
};

fn main() -> std::io::Result<()> {
    let file = File::open("file.xml")?;
    let file = BufReader::new(file); // Buffering is important for performance

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                println!("{:spaces$}{name}: {{", "", spaces = depth * 2);
                for attr in attributes {
                    let name = attr.name.to_string() + ",";
                    let value = attr.value.to_string();
                    println!(
                        "{:spaces$}{{ name: {name:5} value: {value} }}",
                        "",
                        spaces = 1 + depth * 4
                    );
                }
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                if name != OwnedName::from_str("testcases").unwrap() {
                    println!("{:spaces$}}},", "", spaces = depth * 2);
                } else {
                    println!("{:spaces$}}}", "", spaces = depth * 2);
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }

    Ok(())
}
