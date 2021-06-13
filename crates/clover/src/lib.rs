pub mod frontend;
pub mod intermediate;
pub mod backend;
pub mod runtime;

#[cfg(test)]
mod tests {
    use crate::runtime::state::State;
    use crate::runtime::create_state_by_filename;
    use crate::runtime::object::Object;

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

        if let Object::Boolean(value) = object{
            assert!(value, "result is not bool when executing function [{}] in [{}]", function_name, &state.get_program().file_info.as_ref().unwrap().filenames[0]);
        } else {
            panic!("result is not bool when executing function [{}] in [{}]", function_name, &state.get_program().file_info.as_ref().unwrap().filenames[0]);
        };
    }

    fn execute(filename: &str, function_names: &[ &str ]) {
        let result = create_state_by_filename(filename);

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
        execute("tests/for_loop.luck", &[ "simple", "nests", "break_loop", "array" ]);
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
}
