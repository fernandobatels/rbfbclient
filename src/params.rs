//!
//! Params utils
//!

use rsfbclient_core::*;
use rutie::*;
use rutie::types::*;

/// Convert the objects to Vec<Param>
/// supported by the rsfbclient
pub trait ToParams {

    fn to_params(self) -> Vec<Param>;
}

impl ToParams for Result<Array, AnyException> {

    fn to_params(self) -> Vec<Param> {
        match self {
            Ok(params) => params.to_params(),
            _ => vec![]
        }
    }
}

impl ToParams for Array {
    fn to_params(self) -> Vec<Param> {
        let mut query_params = vec![];

        for (i, param) in self.into_iter().enumerate() {

            match param.ty() {
                ValueType::RString => {
                    let pstr = RString::try_convert(param)
                        .unwrap()
                        .to_string();
                    query_params.push(pstr.into_param());
                },
                ValueType::Fixnum => {
                    let pstr = param.try_convert_to::<Fixnum>()
                        .unwrap()
                        .to_i64();
                    query_params.push(pstr.into_param());
                },
                ValueType::Float => {
                    let pstr = param.try_convert_to::<Float>()
                        .unwrap()
                        .to_f64();
                    query_params.push(pstr.into_param());
                },
                ValueType::Nil => {
                    query_params.push(None::<i32>.into_param());
                },
                ValueType::False => {
                    query_params.push(true.into_param());
                },
                ValueType::True => {
                    query_params.push(false.into_param());
                },
                _ => {
                    VM::raise(Class::from_existing("StandardError"), &format!("Unsuported type({:?}) param at {}", param.ty(), i));
                }
            }
        }

        return query_params;
    }
}
