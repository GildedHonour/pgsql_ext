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

//NOTE: refactoring will be required once all works well

#[no_mangle]
pub extern "C" fn ex4_test(fcinfo: FunctionCallInfo) -> Datum {
  let res = unsafe {
    let trig_data: *const TriggerData = (*fcinfo).context as *const TriggerData;
    if !trig_data.is_null() {
      println!("[RUST_DEBUG]: called as a trigger");

      let ret_tuple: HeapTuple = (*trig_data).tg_trigtuple;
      let tup_desc: TupleDesc = (*(*trig_data).tg_relation).rd_att;

      let my_uuid = Uuid::new_v4();
      let mut f = File::create(format!("/Users/alex/projects/rust/pgsql__workspace/rust_lang_ext__workspace/pgsql_ext_core/data/{}.txt", my_uuid)).unwrap();

      let col_num = (*tup_desc).natts;
      for x in 0..col_num {

        //1 - column name
        let col_name: String = (*tup_desc).attrs.
          as_slice(col_num as usize)[x as usize].attname.data.
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
        let col_type: &CStr = CStr::from_ptr(SPI_gettype(tup_desc, x + 1));
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
            let col_val_ptr: Datum = SPI_getbinval(ret_tuple, tup_desc, x + 1, &mut is_null);
            if !is_null {
              let a1 = get_var_size_4b(col_val_ptr);
              println!("VARSIZE_4B {:?}", a1);

                      // let possible_size: usize = 10;
                      // let a1 = std::slice::from_raw_parts(col_val_ptr2, possible_size);
                      // println!("a1: {:?}", a1);
                      // println!("a1 len: {:?}", a1.len());

            
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
            let maybe_val = SPI_getvalue(ret_tuple, tup_desc, x + 1);
            let s3 = if !maybe_val.is_null() {
              let col_val: &CStr = CStr::from_ptr(maybe_val);
              format!("column_value: {:?}", col_val)
            } else {
              format!("column_value: null")
            };

            println!("{}", s3);
            f.write_all(s3.as_bytes());
          }
        }

        println!("\r\n");
        f.write_all(b"\r\n\r\n");
      }

      ret_tuple
    } else {
      //TODO here 'elog' should be used instead, but it doesn't exist in bindings.rs
      panic!("[RUST_ERROR] calling it as a trigger --> why u no do it!");
    }
  };

  res as Datum
}


// implementation of C macro:
// #define VARSIZE_4B(PTR)  ((((varattrib_4b *) (PTR))->va_4byte.va_header >> 2) & 0x3FFFFFFF)
const SHIFT_VAL1: u32  = 2;
const SHIFT_VAL2: u32 = 0x3FFFFFFF;
fn get_var_size_4b(ptr: Datum) -> u32 {
  //TODO refactor

  let mut ptr2;
  let ptr3 = unsafe {
      ptr2 = std::ptr::NonNull::new(
        ptr as *mut varattrib_4b
      ).unwrap();
      ptr2.as_mut()
  };

  // NOTE it works as is
  // if something looks off, try to use pointer de-refferencing instead
  // (*ptr3).va_4byte

  let ptr33 = unsafe {
    ptr3.va_4byte.as_ref()
  };
  (ptr33.va_header >> SHIFT_VAL1) &  SHIFT_VAL2
}