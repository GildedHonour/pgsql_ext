#[macro_use]
extern crate pgxr2;
extern crate uuid;
extern crate base64;

use std::fs::File;
use std::io::Write;
use std::ffi::CStr;
use std::str;

use uuid::Uuid;
use base64::{encode, decode};
use pgxr2::bindings::*;
use pgxr2::*;

PG_MODULE_MAGIC!();
PG_FUNCTION_INFO_V1!(pg_finfo_ex4_test);

#[no_mangle]
pub extern "C" fn ex4_test(fcinfo: FunctionCallInfo) -> Datum {
  // let res = unsafe {
    let trig_data: *const TriggerData = unsafe { (*fcinfo).context as *const TriggerData};
    if trig_data.is_null() {
      //TODO here 'elog' should be used instead, but it doesn't exist in bindings.rs

      panic!("[RUST_ERROR] calling it as a trigger --> why u no do it!");

    }
      println!("[RUST_DEBUG]: called as a trigger");

    let ret_tuple: HeapTuple = unsafe { (*trig_data).tg_trigtuple };
    let rel: Relation = unsafe { (*trig_data).tg_relation };
    let tup_desc: TupleDesc = unsafe {(*rel).rd_att};

    let my_uuid = Uuid::new_v4();
    let mut f = File::create(format!("/tmp/data/{}.txt", my_uuid)).unwrap();

    let r =  unsafe { CStr::from_ptr(SPI_getrelname(rel))};
    f.write_all(format!("{:?}\n", r).as_bytes());

      let col_num = unsafe { (*tup_desc).natts };
      for x in 0..col_num {

        //1 - column name
        let att = unsafe {
          (*tup_desc).attrs.
              as_slice(col_num as usize)[x as usize].attname
        };
        let col_name: String = att.data.
            iter().
            map(|x| *x as u8).
            filter(|x| *x != 0).
            map(|x| x as char).
            collect();
        let s1 = format!("column_name: {:?}", col_name);
        println!("{}", s1);
        f.write_all(s1.as_bytes());
        f.write_all(b"\r\n");


        //2 - column type
        let col_type: &CStr = unsafe { CStr::from_ptr(SPI_gettype(tup_desc, x + 1)) };
        let s2 = format!("column_type: {:?}", col_type);
        println!("{}", s2);
        f.write_all(s2.as_bytes());
        f.write_all(b"\r\n");


        //3 - column value
        let col_type_str_slice: &str = col_type.to_str().unwrap();
        match col_type_str_slice {
          _ if col_type_str_slice == "bytea" => {
            println!("column_type == bytea");
            let mut is_null: bool = true;
            let col_val_ptr: Datum = unsafe { SPI_getbinval(ret_tuple, tup_desc, x + 1, &mut is_null) };
            if !is_null {

              //TODO
              let col_val_ptr2 = &col_val_ptr as *const _;
              let possible_size: usize = 10;
              let a1 = unsafe { std::slice::from_raw_parts(col_val_ptr2, possible_size) };
              println!("a1: {:?}", a1);
              println!("a1 len: {:?}", a1.len());
              // // base64
              // let a = b"hello world";
              // let b = "aGVsbG8gd29ybGQ=";

              // assert_eq!(encode(a), b);
              // assert_eq!(a, &decode(b).unwrap()[..]);


              // let b64_val = base64::encode(col_val);
              // let s3 = format!("column_value: {:?}", b64_val);
              // f.write_all(s3.as_bytes());
            } else {
              println!("column_value is null");
            }
          },
          _ => {
            println!("column_type == {}", col_type_str_slice);
            let col_val: &CStr = unsafe { CStr::from_ptr(SPI_getvalue(ret_tuple, tup_desc, x + 1)) };
            let s3 = format!("column_value: {:?}", col_val);
            println!("{}", s3);
            f.write_all(s3.as_bytes());
          }
        }

        println!("\r\n");
        f.write_all(b"\r\n\r\n");
      }

  ret_tuple as Datum
}
