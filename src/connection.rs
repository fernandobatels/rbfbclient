//!
//! Connection class
//!

use rsfbclient::*;
use rutie::*;
use std::sync::Mutex;
use crate::params::ToParams;
use crate::rows::ToRows;

class!(Connection);

pub struct ConnectionData {
    pub conn: Mutex<Option<rsfbclient::Connection<rsfbclient_native::NativeFbClient>>>
}

wrappable_struct!(ConnectionData, ConnectionDataWrapper, CD_WRAPPER);

methods!(
    Connection,
    itself,

    fn new(args: Hash) -> AnyObject {
        let args = args.map_err(|e| VM::raise_ex(e)).unwrap();

        let mut cb = ConnectionBuilder::linked();

        if let Ok(host) = args.at(&Symbol::new("host")).try_convert_to::<RString>() {
            cb.host(host.to_string());
        }

        if let Ok(port) = args.at(&Symbol::new("port")).try_convert_to::<Integer>() {
            cb.port(port.to_i32() as u16);
        }

        if let Ok(db_name) = args.at(&Symbol::new("db_name")).try_convert_to::<RString>() {
            cb.db_name(db_name.to_string());
        }

        if let Ok(user) = args.at(&Symbol::new("user")).try_convert_to::<RString>() {
            cb.user(user.to_string());
        }

        if let Ok(pass) = args.at(&Symbol::new("pass")).try_convert_to::<RString>() {
            cb.pass(pass.to_string());
        }

        match cb.connect() {
            Ok(conn) => {
                let data = ConnectionData {
                    conn: Mutex::new(Some(conn)),
                };

                Class::from_existing("Connection").wrap_data(data, &*CD_WRAPPER)
            }
            Err(e) => {
                VM::raise(Class::from_existing("StandardError"), &e.to_string());

                NilClass::new().to_any_object()
            }
        }
    }

    fn close() -> NilClass {

        let take_conn = itself.get_data_mut(&*CD_WRAPPER)
            .conn
            .get_mut()
            .map_err(|e| VM::raise(Class::from_existing("StandardError"), &e.to_string()))
            .unwrap()
            .take();

        if take_conn.is_none() {
            VM::raise(Class::from_existing("StandardError"), "Connection is already closed");
            return NilClass::new();
        }

        let conn = take_conn.unwrap();

        if let Err(e) = conn.close() {
            VM::raise(Class::from_existing("StandardError"), &e.to_string());
        }

        NilClass::new()
    }

    fn execute(query: RString, params: Array) -> NilClass {
        let op_conn = itself.get_data_mut(&*CD_WRAPPER)
            .conn
            .get_mut()
            .map_err(|e| VM::raise(Class::from_existing("StandardError"), &e.to_string()))
            .unwrap()
            .as_mut();

        if op_conn.is_none() {
            VM::raise(Class::from_existing("StandardError"), "Connection is closed");
            return NilClass::new();
        }

        let conn = op_conn.unwrap();

        let query = query.map_err(|e| VM::raise_ex(e))
            .unwrap()
            .to_string();

        conn.execute(&query, params.to_params())
            .map_err(|e| VM::raise(Class::from_existing("StandardError"), &e.to_string()))
            .unwrap();

        NilClass::new()
    }

    fn query(as_hash: Boolean, query: RString, params: Array) -> Array {
        let op_conn = itself.get_data_mut(&*CD_WRAPPER)
            .conn
            .get_mut()
            .map_err(|e| VM::raise(Class::from_existing("StandardError"), &e.to_string()))
            .unwrap()
            .as_mut();

        if op_conn.is_none() {
            VM::raise(Class::from_existing("StandardError"), "Connection is closed");
            return Array::new();
        }

        let conn = op_conn.unwrap();

        let as_hash = as_hash.map_err(|e| VM::raise_ex(e))
            .unwrap()
            .to_bool();

        let query = query.map_err(|e| VM::raise_ex(e))
            .unwrap()
            .to_string();

        let rows = conn.query(&query, params.to_params())
            .map_err(|e| VM::raise(Class::from_existing("StandardError"), &e.to_string()))
            .unwrap();

        if as_hash {
            rows.to_hash_rows()
        } else {
            rows.to_array_rows()
        }
    }
);

pub fn defs(itself: &mut Class) {
    itself.def_self("new", new);
    itself.def("close", close);
    itself.def("_execute", execute);
    itself.def("_query", query);
}
