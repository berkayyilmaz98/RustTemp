extern crate quick_xml;
extern crate serde;

use quick_xml::de::from_str;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

fn main() {

    let processing_file = "file.xml";
    println!("Will attempt to process file: '{}'", &processing_file);

    // Try to load the contents of the file
    let file_content : String = match std::fs::read_to_string(&processing_file) {
        Ok(file_content) => file_content,
        Err(e) => {
            panic!("Failed to read file: '{}' -- {}", &processing_file, e);
        }
    };

    // Now, try to deserialize the XML we have in file_content
    let defect_list : Item = from_str(&file_content).unwrap();

    // Assuming the unwrap above didn't blow up, we should get a count here
    println!("Retrieved name: {} from file '{}'", defect_list.name, &processing_file);
}
