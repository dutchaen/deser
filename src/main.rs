mod helper;
mod serializer;

use once_cell::sync::Lazy;
use common_macros::hash_map;
use std::collections::HashMap;
use clipboard_win::{Clipboard, Getter, Setter, formats::Unicode};

type SerializeFn = fn(&serde_json::Value, &str, &mut String, &mut Vec<String>) -> ();

#[allow(non_upper_case_globals)]
static function_map: Lazy<HashMap<&str, SerializeFn>> = Lazy::new(|| {
    hash_map! {
        "Golang" => serializer::golang as SerializeFn,
        "Rust" => serializer::rust as SerializeFn,
        "C#" => serializer::csharp as SerializeFn
    }
});

fn main() {
    let mut text = String::new();
    let _clip = Clipboard::new_attempts(10).expect("Open clipboard");
    match Unicode.read_clipboard(&mut text) {
        Ok(_) => {},
        Err(e) => panic!("There was an error reading from the clipboard: {:?}", e),
    };

    let json_object = match serde_json::from_str(&text) {
        Ok(json_object) => json_object,
        Err(e) => panic!("There was an error parsing JSON from your clipboard: {:?}", e)
    };

    println!("[!] Successfully parsed JSON structure!");

    for (index, (name, _)) in function_map.iter().enumerate() {
        println!("[{}] {}", index, name);
    }
    eprint!("Which one? >> ");
    let selection = read_input::shortcut::input_inside(0..function_map.len());

    eprint!("Name for initial structure? >> ");
    let struct_name = read_input::shortcut::simple_input::<String>();

    if let Some((name, func)) = function_map.iter().nth(selection) {
        let mut result = String::new();
        let mut used_structs: Vec<String> = vec![];

        func(&json_object, &struct_name, &mut result, &mut used_structs);

        println!("Finished deserializing into {} structures...", name);
        match Unicode.write_clipboard(&result) {
            Ok(_) => println!("Successfully copied data to the clipboard."),
            Err(e) => panic!("There was an error copying the JSON to your clipboard: {:?}", e)
        };
    }
}
