use log::{Level, warn, error, debug, info, trace};

#[cfg(test)]
mod tests;

pub fn entry(classfile: String) -> (String, i32, i32){
    println!("In entry: files: {}", classfile);
    return ("".to_string(), 0, 0);
}

