use clover::runtime::state::State;
use clover::runtime::object::Object;
use clover::runtime::program::RuntimeError;

pub fn print(_state: &mut State, parameters: &[ Object ]) -> Result<Object, RuntimeError> {

    for object in parameters {
        print!("{}", object.to_string());
    };

    println!();

    Ok(Object::Null)
}