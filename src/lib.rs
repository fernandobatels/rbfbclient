//!
//! Ruby Firebird Client
//!

#[macro_use]
extern crate rutie;

use rutie::{Class, Object, RString, VM};

class!(Rbfbclient);

methods!(
    Rbfbclient,
    _itself,
    fn perform(input: RString) -> RString {

        RString::new_utf8("???")
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_rbfbclient() {
    Class::from_existing("Rbfbclient").define(|itself| {
        itself.def_self("teste", perform);
    });
}
