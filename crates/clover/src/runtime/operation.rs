use crate::runtime::object::{Object, Reference, ModelInstance};
use crate::runtime::program::RuntimeError;
use crate::runtime::opcode::OPERATION_EQUAL;
use crate::runtime::state::State;

const META_METHODS: &[ &str ] = &[ "_add", "_sub", "_mul", "_div", "_mod", "_eq", "_gt", "_lt", "_gte", "_lte" ];

impl State {

    fn integer_add(&self, left: i64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Integer(value) => Ok(Object::Integer(left + value)),
            Object::Float(_) => self.float_add(left as f64, right),
            Object::String(value) => Ok(Object::String(left.to_string() + value)),

            _ => Err(RuntimeError::new("can not add integer with object", self.last_position()))
        }
    }

    fn integer_sub(&self, left: i64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Integer(value) => Ok(Object::Integer(left - value)),
            Object::Float(_) => self.float_sub(left as f64, right),

            _ => Err(RuntimeError::new("can not sub integer with object", self.last_position()))
        }
    }

    fn integer_mul(&self, left: i64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Integer(value) => Ok(Object::Integer(left * value)),
            Object::Float(_) => self.float_mul(left as f64, right),

            _ => Err(RuntimeError::new("can not sub integer with object", self.last_position()))
        }
    }

    fn integer_div(&self, left: i64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Integer(value) => {
                if *value == 0 {
                    Err(RuntimeError::new("divide by zero", self.last_position()))
                } else {
                    Ok(Object::Integer(left / value))
                }
            },
            Object::Float(_) => self.float_div(left as f64, right),

            _ => Err(RuntimeError::new("can not sub integer with object", self.last_position()))
        }
    }

    fn integer_mod(&self, left: i64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Integer(value) => Ok(Object::Integer(left % value)),
            Object::Float(_) => self.float_mod(left as f64, right),

            _ => Err(RuntimeError::new("can not sub integer with object", self.last_position()))
        }
    }

    fn integer_eq(&self, left: i64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Integer(value) => Ok(Object::Boolean(left == *value)),
            Object::Float(_) => self.float_eq(left as f64, right),

            _ => Err(RuntimeError::new("can not sub integer with object", self.last_position()))
        }
    }

    fn integer_gt(&self, left: i64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Integer(value) => Ok(Object::Boolean(left > *value)),
            Object::Float(_) => self.float_gt(left as f64, right),

            _ => Err(RuntimeError::new("can not sub integer with object", self.last_position()))
        }
    }

    fn integer_lt(&self, left: i64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Integer(value) => Ok(Object::Boolean(left < *value)),
            Object::Float(_) => self.float_lt(left as f64, right),

            _ => Err(RuntimeError::new("can not sub integer with object", self.last_position()))
        }
    }

    fn integer_gte(&self, left: i64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Integer(value) => Ok(Object::Boolean(left >= *value)),
            Object::Float(_) => self.float_gte(left as f64, right),

            _ => Err(RuntimeError::new("can not sub integer with object", self.last_position()))
        }
    }

    fn integer_lte(&self, left: i64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Integer(value) => Ok(Object::Boolean(left <= *value)),
            Object::Float(_) => self.float_lte(left as f64, right),

            _ => Err(RuntimeError::new("can not sub integer with object", self.last_position()))
        }
    }

    fn integer_operation(&self, left: i64, right: &Object, operand: usize) -> Result<Object, RuntimeError> {
        match operand {
            0 => self.integer_add(left, right),
            1 => self.integer_sub(left, right),
            2 => self.integer_mul(left, right),
            3 => self.integer_div(left, right),
            4 => self.integer_mod(left, right),
            5 => self.integer_eq(left, right),
            6 => self.integer_gt(left, right),
            7 => self.integer_lt(left, right),
            8 => self.integer_gte(left, right),
            9 => self.integer_lte(left, right),

            _ => Err(RuntimeError::new("unknown operation", self.last_position()))
        }
    }

    fn float_add(&self, left: f64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Float(value) => Ok(Object::Float(left + value)),
            Object::Integer(value) => Ok(Object::Float(left + *value as f64)),
            Object::String(value) => Ok(Object::String(left.to_string() + value)),

            _ => Err(RuntimeError::new("can not add float with object", self.last_position()))
        }
    }

    fn float_sub(&self, left: f64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Float(value) => Ok(Object::Float(left - value)),
            Object::Integer(value) => Ok(Object::Float(left - *value as f64)),

            _ => Err(RuntimeError::new("can not sub float with object", self.last_position()))
        }
    }

    fn float_mul(&self, left: f64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Float(value) => Ok(Object::Float(left * value)),
            Object::Integer(value) => Ok(Object::Float(left * *value as f64)),

            _ => Err(RuntimeError::new("can not sub float with object", self.last_position()))
        }
    }

    fn float_div(&self, left: f64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Float(value) => Ok(Object::Float(left / value)),
            Object::Integer(value) => Ok(Object::Float(left / *value as f64)),

            _ => Err(RuntimeError::new("can not sub float with object", self.last_position()))
        }
    }

    fn float_mod(&self, left: f64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Float(value) => Ok(Object::Float(left % value)),
            Object::Integer(value) => Ok(Object::Float(left % *value as f64)),

            _ => Err(RuntimeError::new("can not sub float with object", self.last_position()))
        }
    }

    fn float_eq(&self, left: f64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Float(value) => Ok(Object::Boolean(left == *value)),
            Object::Integer(value) => Ok(Object::Boolean(left == *value as f64)),

            _ => Err(RuntimeError::new("can not sub float with object", self.last_position()))
        }
    }

    fn float_gt(&self, left: f64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Float(value) => Ok(Object::Boolean(left > *value)),
            Object::Integer(value) => Ok(Object::Boolean(left > *value as f64)),

            _ => Err(RuntimeError::new("can not sub float with object", self.last_position()))
        }
    }

    fn float_lt(&self, left: f64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Float(value) => Ok(Object::Boolean(left < *value)),
            Object::Integer(value) => Ok(Object::Boolean(left < *value as f64)),

            _ => Err(RuntimeError::new("can not sub float with object", self.last_position()))
        }
    }

    fn float_gte(&self, left: f64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Float(value) => Ok(Object::Boolean(left >= *value)),
            Object::Integer(value) => Ok(Object::Boolean(left >= *value as f64)),

            _ => Err(RuntimeError::new("can not sub float with object", self.last_position()))
        }
    }

    fn float_lte(&self, left: f64, right: &Object) -> Result<Object, RuntimeError> {
        match right {
            Object::Float(value) => Ok(Object::Boolean(left <= *value)),
            Object::Integer(value) => Ok(Object::Boolean(left <= *value as f64)),

            _ => Err(RuntimeError::new("can not sub float with object", self.last_position()))
        }
    }

    fn float_operation(&self, left: f64, right: &Object, operand: usize) -> Result<Object, RuntimeError> {
        match operand {
            0 => self.float_add(left, right),
            1 => self.float_sub(left, right),
            2 => self.float_mul(left, right),
            3 => self.float_div(left, right),
            4 => self.float_mod(left, right),
            5 => self.float_eq(left, right),
            6 => self.float_gt(left, right),
            7 => self.float_lt(left, right),
            8 => self.float_gte(left, right),
            9 => self.float_lte(left, right),

            _ => Err(RuntimeError::new("unknown operation", self.last_position()))
        }
    }

    fn string_operation(&self, left: &str, right: &Object, operand: usize) -> Result<Object, RuntimeError> {
        match operand {
            0 => {
                match right {
                    Object::String(_) | Object::Integer(_) | Object::Float(_) | Object::Boolean(_) | Object::Null => Ok(Object::String(left.to_string() + &right.to_string())),
                    _ => Err(RuntimeError::new("can not add string with object", self.last_position()))
                }
            },

            _ => Err(RuntimeError::new("unknown operation", self.last_position()))
        }
    }

    fn model_instance_operation(&mut self, left: Reference<ModelInstance>, right: &Object, operand: usize) -> Result<(), RuntimeError> {
        if operand >= META_METHODS.len() {
            return Err(RuntimeError::new("unknown operation", self.last_position()));
        };

        let meta_method_name = META_METHODS[operand];

        let meta_method_index = if let Some(index) = self.get_program().models[left.borrow().model_index].functions.get(meta_method_name) {
            *index
        } else {
            return Err(RuntimeError::new("meta method does not exists", self.last_position()));
        };

        self.call_function_by_index(meta_method_index, &[ Object::Instance(left.clone()), right.clone() ])
    }

    pub fn binary_operation_with_parameters(self: &mut State, left: &Object, right: &Object, operand: usize) -> Result<(), RuntimeError> {
        if operand & 256 > 0 {
            self.push(match operand & 255 {
                // and
                1 => Object::Boolean(left.to_bool() && right.to_bool()),
                // or
                2 => Object::Boolean(left.to_bool() || right.to_bool()),

                _ => { return Err(RuntimeError::new("unknown operation", self.last_position())); }
            });

            return Ok(());
        };

        if let Object::Instance(model_instance) = left {
            return self.model_instance_operation(model_instance.clone(), right, operand);
        };

        self.push(match left {
            Object::Integer(value) => self.integer_operation(*value, right, operand)?,
            Object::Float(value) => self.float_operation(*value, right, operand)?,
            Object::String(value) => self.string_operation(value, right, operand)?,

            Object::Null => {
                if operand == OPERATION_EQUAL {
                    self.push(Object::Boolean(right.is_null()));
                    return Ok(());
                } else {
                    return Err(RuntimeError::new("null can not do this kind of operation", self.last_position()));
                };
            }

            _ => { return Err(RuntimeError::new("unknown object", self.last_position())); }
        });

        Ok(())
    }

    pub fn negative_operation(&self, target: &Object) -> Result<Object, RuntimeError> {
        match target {
            Object::Integer(value) => Ok(Object::Integer(-*value)),
            Object::Float(value) => Ok(Object::Float(-*value)),

            _ => Err(RuntimeError::new("object can not do minus operation", self.last_position()))
        }
    }
}