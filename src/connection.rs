//!
//! Connection class
//!

use rutie::{Class, Object, RString, VM, AnyObject, NilClass};
use rsfbclient::{ConnectionBuilder};
use std::cell::RefCell;

class!(Connection);

pub struct ConnectionData {
    pub conn: RefCell<rsfbclient::Connection<rsfbclient_native::NativeFbClient>>
}

wrappable_struct!(ConnectionData, ConnectionDataWrapper, CD_WRAPPER);

methods!(
    Connection,
    itself,

    fn new(host: RString, db_name: RString) -> AnyObject {

        let host = host.map_err(|e| VM::raise_ex(e))
            .unwrap()
            .to_string();


        let conn = ConnectionBuilder::linked()
            .host(host)
            .connect()
            .map_err(|e| VM::raise(Class::new("Connection", None), &e.to_string()))
            .unwrap();

        let data = ConnectionData {
            conn: RefCell::new(conn)
        };

        Class::new("Connection", None).wrap_data(data, &*CD_WRAPPER)
    }

    fn close() -> NilClass {
        let conn = itself.get_data_mut(&*CD_WRAPPER).conn.get_mut();
        drop(conn);

        NilClass::new()
    }
);


pub fn defs(itself: &mut Class) {
    itself.def("close", close);
    itself.def_self("new", new);
}

