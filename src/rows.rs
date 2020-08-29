//! Rows and columns utils

use rsfbclient_core::*;
use rutie::*;
use chrono::*;

/// Convert the database rows to a
/// array used in ruby layer
pub trait ToRows {

    fn to_array_rows(self) -> Array;

    fn to_hash_rows(self) -> Array;
}

impl ToRows for Vec<Row> {

    fn to_array_rows(self) -> Array {
        let mut rows = Array::with_capacity(self.len());

        for dbrow in self {
            let dbcols = dbrow.cols;

            let mut row = Array::with_capacity(dbcols.len());

            for (i, dbcol) in dbcols.iter().enumerate() {
                row.push(column_to_object(i, dbcol));
            }

            rows.push(row);
        }

        rows
    }

    fn to_hash_rows(self) -> Array {
        let mut rows = Array::with_capacity(self.len());

        for dbrow in self {
            let mut row = Hash::new();

            for (i, dbcol) in dbrow.cols.iter().enumerate() {
                let col_name = dbcol.name.trim().to_lowercase();
                row.store(RString::new_utf8(&col_name), column_to_object(i, dbcol));
            }

            rows.push(row);
        }

        rows
    }
}

// Return an AnyObject with the column value
fn column_to_object(icol: usize, dbcol: &Column) -> AnyObject {
    let dbcol = dbcol.clone();
    if let Some(dbcol_type) = dbcol.clone().as_ref() {
        match dbcol_type {
            ColumnType::Text(st) => RString::new_utf8(&st).into(),
            ColumnType::Integer(num) => Integer::new(*num).into(),
            ColumnType::Float(num) => Float::new(*num).into(),
            ColumnType::Timestamp(time) => {

                let ntime: NaiveDateTime = dbcol.to_val()
                    .unwrap();
                let mut params = vec![];
                let date_class = Class::from_existing({
                    if time.timestamp_time > 0 {
                        "DateTime"
                    } else {
                        "Date"
                    }
                });

                if time.timestamp_date > 0 {
                    params.push(Integer::new(ntime.year().into()).to_any_object());
                    params.push(Integer::new(ntime.month().into()).to_any_object());
                    params.push(Integer::new(ntime.day().into()).to_any_object());
                }

                if time.timestamp_time > 0 {
                    params.push(Integer::new(ntime.hour().into()).to_any_object());
                    params.push(Integer::new(ntime.minute().into()).to_any_object());
                    params.push(Integer::new(ntime.second().into()).to_any_object());
                }

                unsafe {
                    date_class.send("new", &params)
                }
            },
            _ => {
                VM::raise(Class::from_existing("StandardError"), &format!("{:?} at {}, is not supported", dbcol, icol));
                NilClass::new().into()
            }
        }
    } else {
        NilClass::new().into()
    }
}
