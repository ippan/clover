use clover::runtime::state::State;
use clover::runtime::object::{Object, NativeModel, NativeModelInstance, make_reference, Reference};
use clover::runtime::program::RuntimeError;
use rand::rngs::ThreadRng;
use clover::intermediate::Position;
use rand::Rng;

pub struct Random;

impl NativeModel for Random {
    fn model_get(&self, key: &str) -> Result<Object, RuntimeError> {
        match key {
            "new" => Ok(Object::NativeFunction(Random::new_random)),
            _ => Ok(Object::Null)
        }
    }
}

impl Random {
    pub fn new_random(_state: &mut State, _parameters: &[ Object ]) -> Result<Object, RuntimeError> {
        let random_instance = RandomInstance {
            random: rand::thread_rng()
        };

        Ok(Object::NativeInstance(make_reference(random_instance)))
    }
}

pub struct RandomInstance {
    pub random: ThreadRng
}

impl NativeModelInstance for RandomInstance {
    fn index_get(&self, _this: Reference<dyn NativeModelInstance>, _index: &Object) -> Result<Object, RuntimeError> {
        Ok(Object::Null)
    }

    fn index_set(&mut self, _this: Reference<dyn NativeModelInstance>, _index: &Object, _value: Object) -> Result<(), RuntimeError> {
        Ok(())
    }

    fn instance_get(&self, this: Reference<dyn NativeModelInstance>, key: &str) -> Result<Object, RuntimeError> {
        match key {
            "next_integer" | "next_float" | "within" | "pick" => Ok(Object::InstanceNativeFunction(this, key.to_string())),
            _ => Err(RuntimeError::new("index does not exists", Position::none()))
        }
    }

    fn instance_set(&mut self, _this: Reference<dyn NativeModelInstance>, _key: &str, _value: Object) -> Result<(), RuntimeError> {
        Ok(())
    }

    fn call(&mut self, _this: Reference<dyn NativeModelInstance>, state: &mut State, key: &str, parameters: &[Object]) -> Result<Object, RuntimeError> {
        match key {
            "next_integer" => self.next_integer(state, parameters),
            "next_float" => self.next_float(state, parameters),
            "within" => self.within(state, parameters),
            "pick" => self.pick(state, parameters),
            _ =>  Err(RuntimeError::new("index does not exists", Position::none()))
        }
    }
}

impl RandomInstance {
    pub fn next_integer(&mut self, _state: &mut State, _parameters: &[ Object ]) -> Result<Object, RuntimeError> {
        Ok(Object::Integer(self.random.gen()))
    }

    pub fn next_float(&mut self, _state: &mut State, _parameters: &[ Object ]) -> Result<Object, RuntimeError> {
        Ok(Object::Float(self.random.gen()))
    }

    pub fn within(&mut self, _state: &mut State, parameters: &[ Object ]) -> Result<Object, RuntimeError> {
        if parameters.len() == 0 {
            return Ok(Object::Null);
        };

        let number = &parameters[0];

        Ok(match number {
            Object::Integer(value) => {
                if *value > 0 {
                    Object::Integer(self.random.gen_range(0..*value))
                } else {
                    Object::Integer(0)
                }
            },
            Object::Float(value) => {
                if *value > 0.0 {
                    Object::Float(self.random.gen_range(0.0..*value))
                } else {
                    Object::Float(0.0)
                }
            },
            _ => Object::Null
        })
    }

    pub fn pick(&mut self, _state: &mut State, parameters: &[ Object ]) -> Result<Object, RuntimeError> {
        if parameters.len() == 0 {
            return Ok(Object::Null);
        };

        let value = &parameters[0];

        match value {
            Object::Array(array) => {
                if array.borrow().len() > 0 {
                    let index = self.random.gen_range(0..array.borrow().len());
                    Ok(array.borrow()[index].clone())
                } else {
                    Ok(Object::Null)
                }
            },

            _ => Ok(Object::Null)
        }

    }
}