//!
//! Connection class
//!

use rsfbclient::ConnectionBuilder;
use rutie::*;
use std::cell::RefCell;

class!(Connection);

pub struct ConnectionData {
    pub conn: RefCell<rsfbclient::Connection<rsfbclient_native::NativeFbClient>>,
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
                    conn: RefCell::new(conn),
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
        let conn = itself.get_data_mut(&*CD_WRAPPER).conn.get_mut();
        drop(conn);

        NilClass::new()
    }
);

pub fn defs(itself: &mut Class) {
    itself.def_self("new", new);
    itself.def("close", close);
}
