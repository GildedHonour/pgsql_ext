#[macro_use] extern crate pgxr_11;
// #[macro_use] extern crate pgxr_12;

extern crate uuid;
extern crate base64;

use std::fs::File;
use std::io::Write;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char};
use std::str;
use std::collections::HashMap;
use std::vec::Vec;

use uuid::Uuid;
use base64::{encode, decode};

use pgxr_11::bindings::*;
use pgxr_11::*;
// use pgxr_12::bindings::*;
// use pgxr_12::*;


const VAR_HEADER_SIZE: usize = std::mem::size_of::<i32>();
const CONFIG_DATA_PREFIX: &str = "my_ext";
const PICTURE_FILE_NAME: &str = "picture1__1071_bytes.svg";
const INITIAL_REL_ID: i32 = -1;

PG_MODULE_MAGIC!();
PG_FUNCTION_INFO_V1!(pg_finfo_ex4_test);


#[no_mangle]
pub extern "C" fn ex4_test(fcinfo: FunctionCallInfo) -> Datum {
  let res = unsafe {
    let trig_data: *const TriggerData = (*fcinfo).context as *const TriggerData;
    if !trig_data.is_null() {
      println!("[RUST_DEBUG]: called as a trigger");

      //
      // read config value
      // postgresql.conf
      // a key has to be in the format: 'prefix.value = 123'
      //
      let cfg_key_raw = CString::new(format!("{}.data_dir_path", CONFIG_DATA_PREFIX)).unwrap();
      let cfg_key: *const c_char = cfg_key_raw.as_ptr() as *const c_char;
      let file_full_path_raw = GetConfigOptionByName(cfg_key, &mut std::ptr::null(), false);

      let file_full_path_c_str: &CStr = unsafe { CStr::from_ptr(file_full_path_raw) };
      let full_data_dir_path: &str = file_full_path_c_str.to_str().unwrap();

      let uuid1 = Uuid::new_v4();
      let mut dump_fl = File::create(format!("{}/{}.txt", full_data_dir_path, uuid1)).unwrap();


      //
      // db, schema, table
      //
      let curr_db_ptr = current_database(fcinfo) as *const c_char;
      let curr_db: &CStr = CStr::from_ptr(curr_db_ptr);

      let schema_ptr = SPI_getnspname((*trig_data).tg_relation);
      let schema: &CStr = CStr::from_ptr(schema_ptr);

      let tbl_ptr = SPI_getrelname((*trig_data).tg_relation);
      let tbl: &CStr = CStr::from_ptr(tbl_ptr);

      //
      //primary keys
      //
      let mut c_odi: Oid = 0;
      let rd_id = (*(*trig_data).tg_relation).rd_id;
      let pkattnos: *mut Bitmapset = get_primary_key_attnos(rd_id, false, &mut c_odi);
      let mut pr_idx_s = Vec::new();
      if !pkattnos.is_null() {
        let mut rel_id_i = bms_next_member(pkattnos, INITIAL_REL_ID);
        while rel_id_i >= 0 {
          let col_idx: i32  = rel_id_i + FirstLowInvalidHeapAttributeNumber;
          pr_idx_s.push(col_idx);
          rel_id_i = bms_next_member(pkattnos , rel_id_i);
        }
      } else {
        println!("get_primary_key_attnos NUL");
      }

      let s = format!("{};{};{};{};\r\n", curr_db.to_str().unwrap(), schema.to_str().unwrap(), tbl.to_str().unwrap(), pr_idx_s.len());
      dump_fl.write_all(s.as_bytes());

      let ret_tuple: HeapTuple = (*trig_data).tg_trigtuple;
      let tup_desc: TupleDesc = (*(*trig_data).tg_relation).rd_att;

      let col_num = (*tup_desc).natts;
      let mut pr_s = HashMap::new();

      for x in 0..col_num {
        //
        //1 - column name
        //
        let col_name: String = (*tup_desc).attrs.
          as_slice(col_num as usize)[x as usize].attname.data.
          iter().
          map(|x| *x as u8).
          filter(|x| *x != 0).
          map(|x| x as char).
          collect();

        // TODO debug
        // let s1 = format!("column_name: {:?}", col_name);
        // println!("{}", s1);
        // dump_fl.write_all(s1.as_bytes());
        // dump_fl.write_all(b"\r\n");


        //
        //2 - column type
        //
        let col_type: &CStr = CStr::from_ptr(SPI_gettype(tup_desc, x + 1));

        // TODO debug
        // let s2 = format!("column_type: {:?}", col_type);
        // println!("{}", s2);
        // dump_fl.write_all(s2.as_bytes());
        // dump_fl.write_all(b"\r\n");


        //
        //3 - column value
        //
        let col_type_str_slice: &str = col_type.to_str().unwrap();
        match col_type_str_slice {
          _ if col_type_str_slice == "bytea" => {
            println!("column_type == bytea");
            let mut is_null: bool = true;
            let col_val_ptr: Datum = SPI_getbinval(ret_tuple, tup_desc, x + 1, &mut is_null);
            if !is_null {
              let sz = get_var_size_4b(col_val_ptr);
              let data_sz = sz - VAR_HEADER_SIZE;
              let bin_data_ptr: *const u8 = get_var_data_4b(col_val_ptr);

              let bin_data_slice = ::std::slice::from_raw_parts(bin_data_ptr, data_sz);
              let b64_val = base64::encode(bin_data_slice);
              dump_fl.write_all(b64_val.as_bytes());
              dump_fl.write_all(b"\r\n");
            }
          },
          _ => {
            let maybe_val = SPI_getvalue(ret_tuple, tup_desc, x + 1);
            let col_val = if !maybe_val.is_null() {
              let col_val_str: &CStr = CStr::from_ptr(maybe_val);
              Some(col_val_str.to_str().unwrap())
            } else {
              None
            };

            if pr_idx_s.len() > 0 {
              if pr_idx_s.iter().any(|it| *it == (x + 1)) {
                pr_s.insert(col_name, col_val);
              }
            }
          }
        }
      }

      for (k, v) in pr_s.iter() {
        let v2 = match v {
          Some(x) => x,
          None => "?"
        };

        dump_fl.write_all(format!("{}={};\r\n", k, v2).as_bytes());
      }

      dump_fl.write_all(b"\r\n");
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

fn get_var_size_4b(ptr: Datum) -> usize {
  let mut _ptr1;
  let ptr1 = unsafe {
      _ptr1 = std::ptr::NonNull::new(ptr as *mut varattrib_4b).unwrap();
      _ptr1.as_mut()
  };

  // this works as is
  // but if something begins to looks off, try to use pointer de-refferencing instead:
  // (*ptr1).va_4byte
  let ptr2 = unsafe {
    //bindings.rs#123 -->  __BindgenUnionField
    ptr1.va_4byte.as_ref()
  };

  ((ptr2.va_header >> SHIFT_VAL1) &  SHIFT_VAL2) as usize
}

fn get_var_data_4b(ptr: Datum) -> *const u8 {
  let mut _ptr1;
  let ptr1 = unsafe {
      _ptr1 = std::ptr::NonNull::new(ptr as *mut varattrib_4b).unwrap();
      _ptr1.as_mut()
  };

  let ptr2 = unsafe {
    // bindings.ru#11234 -> varattrib_4b
    ptr1.va_4byte.as_ref()
  };

  let ptr3 = unsafe {
  // bindings.ru#88 -> __IncompleteArrayField
    ptr2.va_data.as_ptr()
  };

  ptr3 as *const u8
}
