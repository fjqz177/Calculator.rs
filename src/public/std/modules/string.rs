use std::collections::VecDeque;

use crate::public::run_time::build_in::BuildInFnIdenti;
use crate::public::run_time::scope::Scope;
use crate::public::std::utils::get_self_prop::get_self_prop;
use crate::public::value::array::ArrayLiteral;
use crate::public::value::function::{
    BuildInFnParam, BuildInFunction, Function, Overload as FunctionOverLoad,
};
use crate::public::value::oop::class::{Class, Property};
use crate::public::value::value::{Overload as ValueOverload, Value, ValueType};

use super::super::utils::get_val::get_val;
use super::BuildInFnCall;

#[derive(PartialEq, Clone)]
pub enum StringFn {
    SPLIT,
    REPLACE,
    REPEAT,
    JOIN,
    STARTWITH,
    ENDWITH,
}

pub fn module_class() -> Class {
    let split = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::String, "divider"),
        ],
        identi: BuildInFnIdenti::String(StringFn::SPLIT),
    };
    let replace = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Void, "from"),
            BuildInFnParam(ValueType::Void, "to"),
        ],
        identi: BuildInFnIdenti::String(StringFn::REPLACE),
    };
    let repeat = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::Void, "num"),
        ],
        identi: BuildInFnIdenti::String(StringFn::REPEAT),
    };
    let join = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::String, "divider"),
        ],
        identi: BuildInFnIdenti::String(StringFn::JOIN),
    };
    let start_with = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::String, "str"),
        ],
        identi: BuildInFnIdenti::String(StringFn::STARTWITH),
    };
    let end_with = BuildInFunction {
        params: vec![
            BuildInFnParam(ValueType::Object, "self"),
            BuildInFnParam(ValueType::String, "str"),
        ],
        identi: BuildInFnIdenti::String(StringFn::ENDWITH),
    };

    return Class::new(
        vec![Property(ValueType::String, String::from("v"))],
        vec![
            (String::from("split"), Function::create(split)),
            (String::from("replace"), Function::create(replace)),
            (String::from("repeat"), Function::create(repeat)),
            (String::from("join"), Function::create(join)),
            (String::from("start_with"), Function::create(start_with)),
            (String::from("end_with"), Function::create(end_with)),
        ],
    );
}

impl BuildInFnCall for StringFn {
    fn call(&self, scope: &mut Scope) -> Result<Value, ()> {
        let self_value = get_val("self", scope)?;
        let str_value = get_self_prop(&self_value, "v")?;
        let Value::String(str_ref) = str_value else {
            return Ok(Value::create(String::new()));
        };

        let result = match self {
            StringFn::SPLIT => {
                let divider_value = get_val("divider", scope)?;

                if let Value::String(div) = divider_value {
                    let str_ref = str_ref.borrow();
                    let div_ref = div.borrow();
                    // splited chars
                    let res_split = if div_ref.is_empty() {
                        str_ref.split(' ')
                    } else {
                        let first_ch = div_ref.chars().next().unwrap();
                        str_ref.split(first_ch)
                    };
                    // convert splited to VecDeque<String>
                    let mut res_vec = VecDeque::new();
                    for c in res_split {
                        let c_value = Value::create(c.to_string());
                        res_vec.push_back(c_value);
                    }
                    Value::create(res_vec)
                } else {
                    Value::create(ArrayLiteral::new())
                }
            }
            StringFn::REPLACE => {
                let from_value = get_val("from", scope)?;
                let to_value = get_val("to", scope)?;

                let str_ref = str_ref.borrow_mut();
                if let (Value::String(from_str), Value::String(to_str)) = (from_value, to_value) {
                    let from_ref = &*(from_str.borrow());
                    let to_ref = &*(to_str.borrow());

                    let replaced_str = str_ref.replace(from_ref, to_ref);
                    Value::create(replaced_str)
                } else {
                    Value::create(String::new())
                }
            }
            StringFn::REPEAT => todo!(),
            StringFn::JOIN => todo!(),
            StringFn::STARTWITH => todo!(),
            StringFn::ENDWITH => todo!(),
        };
        return Ok(result); 
    }
}
