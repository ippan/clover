use clover::runtime::state::State;
use clover::runtime::object::{Object, NativeModel};
use clover::runtime::program::RuntimeError;

pub struct IO;

impl NativeModel for IO {
    fn model_get(&self, key: &str) -> Result<Object, RuntimeError> {
        match key {
            "print" => Ok(Object::NativeFunction(print)),
            "readline" => Ok(Object::NativeFunction(readline)),
            _ => Ok(Object::Null)
        }
    }
}


pub fn print(_state: &mut State, parameters: &[ Object ]) -> Result<Object, RuntimeError> {

    for object in parameters {
        print!("{}", object.to_string());
    };

    println!();

    Ok(Object::Null)
}

fn readline(state: &mut State, _parameters: &[ Object ]) -> Result<Object, RuntimeError> {
    let mut line = String::new();
    if let Err(error) = std::io::stdin().read_line(&mut line) {
        Err(RuntimeError::new(error.to_string().as_str(), state.last_position()))
    } else {
        Ok(Object::String(line))
    }
}