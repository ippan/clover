use std::collections::HashMap;
use clover::{NativeModel, NativeModelInstance, Object, Reference, State};
use clover::debug::{Position, RuntimeError};
use clover::helper::make_reference;

pub struct Map;

impl NativeModel for Map {
    fn call(&mut self, _state: &mut State, _parameters: &[Object]) -> Result<Object, RuntimeError> {
        Ok(Object::NativeInstance(make_reference(MapInstance(HashMap::new()))))
    }
}

pub struct MapInstance(HashMap<String, Object>);


impl NativeModelInstance for MapInstance {
    fn index_get(&self, _this: Reference<dyn NativeModelInstance>, index: &Object) -> Result<Object, RuntimeError> {
        let key = index.to_string();

        if let Some(object) = self.0.get(&key) {
            Ok(object.clone())
        } else {
            Err(RuntimeError::new("index does not exists", Position::none()))
        }
    }

    fn index_set(&mut self, _this: Reference<dyn NativeModelInstance>, index: &Object, value: Object) -> Result<(), RuntimeError> {
        let key = index.to_string();

        self.0.insert(key, value);

        Ok(())
    }

    fn instance_get(&self, this: Reference<dyn NativeModelInstance>, key: &str) -> Result<Object, RuntimeError> {
        match key {
            "length" => Ok(Object::Integer(self.0.len() as i64)),
            "contain_key" => Ok(Object::InstanceNativeFunction(this, key.to_string())),
            _ => self.index_get(this, &Object::String(make_reference(key.to_string())))
        }
    }

    fn instance_set(&mut self, this: Reference<dyn NativeModelInstance>, key: &str, value: Object) -> Result<(), RuntimeError> {
        match key {
            "length" | "contain_key" => Err(RuntimeError::new(&format!("can not change property [{}]", key), Position::none())),
            _ => self.index_set(this, &Object::String(make_reference(key.to_string())), value)
        }
    }

    fn call(&mut self, _this: Reference<dyn NativeModelInstance>, state: &mut State, key: &str, parameters: &[Object]) -> Result<Object, RuntimeError> {
        match key {
            "contain_key" => {
                // accept one parameter only
                if parameters.len() == 1 {
                    let map_key = parameters[0].to_string();
                    Ok(Object::Boolean(self.0.contains_key(&map_key)))
                } else {
                    Err(RuntimeError::new(&format!("wrong number of parameters, expect 1 got {}", parameters.len()), state.last_position()))
                }
            },
            _ =>  Err(RuntimeError::new("index does not exists", state.last_position()))
        }
    }
}