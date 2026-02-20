use goblin::Object;
use std::path::Path;
use tokio::fs;

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let json_string = r#"{
            "executable_path": "path"
        }"#;

    let json: serde_json::Value = serde_json::from_str(json_string).expect("Invalid JSON format!");

    let path = Path::new(json["executable_path"].as_str().expect("No path provided!"));
    let buffer = std::fs::read(path).expect("Coudl not read buffer");

    match Object::parse(&buffer) {
        Ok(object) => match object {
            Object::Elf(elf) => {
                println!("elf: {:#?}", &elf);
            }
            Object::Unknown(magic) => {
                println!("unknown magic: {:#x}", magic)
            }
            _ => {}
        },
        Err(e) => eprintln!("Error parsing object: {}", e),
    }
}
