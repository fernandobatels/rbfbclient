//!
//! Ruby Firebird Client
//!

#[macro_use]
extern crate rutie;
extern crate rsfbclient;
extern crate rsfbclient_native;
#[macro_use]
extern crate lazy_static;

mod connection;

use rutie::{Module, Object};

module!(Rbfbclient);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_rbfbclient() {
    Module::from_existing("Rbfbclient").define(|itself| {

        itself.define_nested_class("Connection", None).define(|itself| connection::defs(itself));
    });
}
