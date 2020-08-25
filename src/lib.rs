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
mod params;
mod rows;

use rutie::{Class, Object};

module!(Rbfbclient);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_rbfbclient() {

    Class::new("Connection", None).define(|itself| connection::defs(itself));
}
