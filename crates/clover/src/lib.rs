mod frontend;
mod intermediate;
mod backend;
mod runtime;
mod version;

pub use runtime::program::Program;
pub use runtime::state::State;
pub use runtime::object::Object;
pub use runtime::object::NativeModel;
pub use runtime::object::NativeModelInstance;
pub use runtime::object::Reference;

use backend::compiler::DefaultStorage;
use backend::compiler::compile_file;
use std::ops::{Deref, DerefMut};

pub mod helper {
    pub use crate::runtime::object::make_reference;
    pub use crate::backend::compiler::Storage;
}

pub mod debug {
    pub use crate::intermediate::CompileErrorList;
    pub use crate::runtime::program::RuntimeError;
    pub use crate::intermediate::Position;
}

pub struct Clover {
    storage: Box<dyn helper::Storage>
}

impl Clover {
    pub fn new_with_file_loader(storage: Box<dyn helper::Storage>) -> Clover {
        Clover {
            storage
        }
    }

    pub fn new() -> Clover {
        Clover {
            storage: Box::new(DefaultStorage::new())
        }
    }

    pub fn compile_file(&self, filename: &str) -> Result<Program, debug::CompileErrorList> {
        compile_file(filename, self.storage.deref())
    }

    pub fn save_program(&self, filename: &str, program: &Program) -> Result<(), debug::CompileErrorList> {

        let mut writer = self.storage.get_writer(filename)?;

        program.serialize(writer.deref_mut()).unwrap();

        Ok(())
    }

    pub fn load_program(&self, filename: &str) -> Result<Program, debug::CompileErrorList> {
        let mut reader = self.storage.get_reader(filename)?;

        Ok(Program::deserialize(&mut reader).unwrap())
    }

    pub fn create_state_by_filename(&self, filename: &str) -> Result<State, debug::CompileErrorList> {
        let program = self.compile_file(filename)?;

        Ok(program.into())
    }

    pub fn run(&self, program: Program) -> Result<Object, debug::RuntimeError> {
        let mut state: State = program.into();

        state.execute()
    }

}


#[cfg(test)]
mod tests {
    use crate::{Clover, State, Object};

    fn execute_function(state: &mut State, function_name: &str) {
        let mut function_index = None;

        for (i, name) in state.get_program().file_info.as_ref().unwrap().function_names.iter().enumerate() {
            if name != function_name {
                continue;
            };

            function_index = Some(i);
            break;
        };

        assert!(function_index.is_some(), "can not found function [{}] in [{}]", function_name, &state.get_program().file_info.as_ref().unwrap().filenames[0]);

        let result = state.execute_by_function_index(function_index.unwrap(), &[]);

        assert!(result.is_ok(), "error occur when executing function [{}] in [{}]", function_name, &state.get_program().file_info.as_ref().unwrap().filenames[0]);

        let object = result.unwrap();

        if let Object::Boolean(value) = object {
            assert!(value, "result is not true when executing function [{}] in [{}]", function_name, &state.get_program().file_info.as_ref().unwrap().filenames[0]);
        } else {
            panic!("result is not a boolean value when executing function [{}] in [{}]", function_name, &state.get_program().file_info.as_ref().unwrap().filenames[0]);
        };
    }

    fn execute(filename: &str, function_names: &[ &str ]) {
        let clover = Clover::new();

        let result = clover.create_state_by_filename(filename);

        assert!(result.is_ok(), "create state with with file [{}]", filename);

        let mut state = result.unwrap();

        for function_name in function_names {
            execute_function(&mut state, *function_name)
        };
    }

    #[test]
    fn integer_operations() {
        execute("tests/integer_operations.luck", &[ "add", "sub", "multiply", "divide" ]);
    }

    #[test]
    fn for_loop() {
        execute("tests/for_loop.luck", &[ "simple", "nests", "break_loop", "array", "for_model" ]);
    }

    #[test]
    fn error_handling() {
        execute("tests/error_handling.luck", &[ "in_same_function", "in_child_function" ]);
    }

    #[test]
    fn function() {
        execute("tests/function.luck", &[ "recursive", "with_return", "first_class_function", "instance_first_class_function" ]);
    }

    #[test]
    fn include() {
        execute("tests/include.luck", &[ "include_function", "include_with_nickname", "include_model" ]);
    }

    #[test]
    fn model() {
        execute("tests/model.luck", &[ "regular", "with_apply" ]);
    }

    #[test]
    fn local() {
        execute("tests/local.luck", &[ "in_file", "in_file_again", "in_function", "in_scope" ]);
    }

    #[test]
    fn convert() {
        execute("tests/convert.luck", &[ "string_to_integer", "string_to_float", "integer_to_string", "integer_to_float", "float_to_string", "float_to_integer" ]);
    }
}
