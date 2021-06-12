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

        assert!(function_index.is_some());

        let result = state.execute_by_function_index(function_index.unwrap(), &[]);

        assert!(result.is_ok());

        let object = result.unwrap();

        if let Object::Boolean(value) = object{
            assert!(value);
        } else {
            panic!("result is not true");
        };
    }

    fn execute(filename: &str, function_names: &[ &str ]) {
        let result = create_state_by_filename(filename);

        assert!(result.is_ok());

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
        execute("tests/for_loop.luck", &[ "simple", "nests" ]);
    }
}
