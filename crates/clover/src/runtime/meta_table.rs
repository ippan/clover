use crate::runtime::object::Object;
use std::collections::HashMap;
use std::rc::Rc;

pub struct IntegerMetaTable;

impl IntegerMetaTable {
    pub fn add(this: &mut Object, parameters: &[&Object]) {

    }

    pub fn get_meta_table() -> Object {
        let mut meta_table = HashMap::new();



        Object::Map(Rc::new(meta_table))
    }

}