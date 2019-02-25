extern crate lightbeam;

use lightbeam::translate;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use parity_wasm::elements::{deserialize_buffer, Module, FunctionNameSection, ExportSection, Internal};

fn read_to_end<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    if path.as_ref() == Path::new("-") {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        stdin.read_to_end(&mut buffer)?;
    } else {
        let mut file = File::open(path)?;
        file.read_to_end(&mut buffer)?;
    }
    Ok(buffer)
}


/// Resolves a function export's index by name. Can be trivially adjusted for
/// all types of exports.
fn func_export_index_by_name(exports: &ExportSection, field_str: &str) -> Option<u32> {
    if let Some(entry) = exports.entries().iter().find(|e| e.field() == field_str) {
        match entry.internal() {
            Internal::Function(index) => Some(*index),
            _ => None,
        }
    } else {
        None
    }
}

fn get_entrypoint_index(data: &[u8]) -> Result<u32, String> {
    if let Ok(module) = deserialize_buffer::<Module>(&data) {
        //if let FunctionNameSection(fns) = module.names_section().unwrap() {
        Ok(func_export_index_by_name(module.export_section().unwrap(), "main").unwrap())
    } else {
        Err("asdf".to_string())
    }
}

fn maybe_main() -> Result<(), String> {
    let data = read_to_end("test.wasm").map_err(|e| e.to_string())?;
    let entrypoint = get_entrypoint_index(&data).unwrap();
    println!("entrypoint is {}", entrypoint);
    

    let translated = translate(&data).map_err(|e| e.to_string())?;
    let result: u32 = unsafe { translated.execute_func(entrypoint, ()) };

    Ok(())
}

fn main() {
    match maybe_main() {
        Ok(()) => (),
        Err(e) => eprintln!("error: {}", e),
    }
}
