#[macro_use]
extern crate pgxr_11;

// #[macro_use]
// extern crate pgxr_12;
extern crate uuid;
extern crate base64;

use std::fs::File;
use std::io::Write;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char};
use std::str;


use uuid::Uuid;
use base64::{encode, decode};

use pgxr_11::bindings::*;
use pgxr_11::*;

// use pgxr_12::bindings::*;
// use pgxr_12::*;


const OIDOID: Oid = 26;
const VAR_HEADER_SIZE: usize = std::mem::size_of::<i32>();
const CONFIG_DATA_PREFIX: &str = "my_ext";

PG_MODULE_MAGIC!();
PG_FUNCTION_INFO_V1!(pg_finfo_ex4_test);


//NOTE: refactoring will be required once all works well

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
      let cfg_key_raw = CString::new(format!("{}.data_dir_path1", CONFIG_DATA_PREFIX)).unwrap();
      let cfg_key: *const c_char = cfg_key_raw.as_ptr() as *const c_char;
      let file_full_path_raw = GetConfigOptionByName(cfg_key, &mut std::ptr::null(), false);

      let file_full_path_c_str: &CStr = unsafe { CStr::from_ptr(file_full_path_raw) };
      let file_full_path: &str = file_full_path_c_str.to_str().unwrap();
      println!("config > file_full_path: {}", file_full_path);



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
              let sz = get_var_size_4b(col_val_ptr);
              println!("VARSIZE_4B {:?}", sz);
              println!("VARSIZE_4B without header {:?}", sz - VAR_HEADER_SIZE);


              let col_val_ptr2 = &(col_val_ptr + VAR_HEADER_SIZE) as *const usize;
              // let elem_count = sz / std::mem::size_of::<usize>();

              let a11 = std::slice::from_raw_parts(col_val_ptr2 , sz);
              println!("a11 len {:?}", a11.len());


              let a111 = get_var_data_4b(col_val_ptr);
              println!("a111{:?}", a111);


              // re-create image, to reassure that no bytes get lost
              // let file_full_path = format!("/Users/alex/projects/rust/pgsql__workspace/rust_lang_ext__workspace/pgsql_ext_core/data/{}.jpg", my_uuid);


              // let mut f_test1 = File::create(file_full_path).unwrap();
              //   f_test1.write_all(&[x2]);
              // let fp = fopen(&file_full_path, &"w");

              // base64
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

        //4 primary keys
        /*
        let spi_c_res = SPI_connect();
        assert_eq!(spi_c_res, SPI_OK_CONNECT as i32);

        let q: &str = "SELECT a.attname
          FROM pg_index i
          JOIN pg_attribute a ON a.attrelid = i.indrelid
          AND a.attnum = ANY(i.indkey)
          WHERE i.indrelid = $1::regclass
          AND i.indisprimary
          ORDER BY a.attnum
        ";

        let q_c_char2 = CString::new(q).unwrap();
        let mut arg_types: [Oid; 1] = [OIDOID];
        let pk_plan = SPI_prepare(q_c_char2.as_ptr(), 1, arg_types.as_mut_ptr());

        let mut oid1 = (*(*trig_data).tg_relation).rd_id;
        let mut values:[Datum; 1] = [oid1 as Datum];


        let spi_sel_res = SPI_execute_plan(pk_plan, values.as_mut_ptr(), std::ptr::null(), false, 0);
        assert_eq!(spi_sel_res, SPI_OK_SELECT as i32);
        println!("[OK] SPI_OK_SELECT");

        let a1 = (*SPI_tuptable).tupdesc;


        for i in 0..SPI_processed {
          let a2 = (*SPI_tuptable).vals;
          format!("a2: {:?}", a2);
        };


        // exec_result = SPI_exec("????", 0);
        // if ((SPI_processed > 0) && (SPI_tuptable != NULL)) {
        //         elog(NOTICE, "SPI_tuptable is not NULL");
        //         SPI_getvalue(SPI_tuptable->vals[0], SPI_tuptable->tupdesc, 1);
        // }


        let spi_f_res = SPI_finish();
        assert_eq!(spi_f_res, SPI_OK_FINISH as i32);
        */

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
fn get_var_size_4b(ptr: Datum) -> usize {
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
    //bindings.rs#123 -->  __BindgenUnionField
    ptr3.va_4byte.as_ref()
  };

  ((ptr33.va_header >> SHIFT_VAL1) &  SHIFT_VAL2) as usize
}


/*
  #define VARDATA_4B(PTR)   (((varattrib_4b *) (PTR))->va_4byte.va_data)
*/
// FIXME implement
fn get_var_data_4b(ptr: Datum) -> usize {
  //TODO refactor

  let mut ptr2;
  let ptr3 = unsafe {
      ptr2 = std::ptr::NonNull::new(
        ptr as *mut varattrib_4b
      ).unwrap();
      ptr2.as_mut()
  };

  let ptr33 = unsafe {
    // bindings.ru#11234 -> varattrib_4b
    ptr3.va_4byte.as_ref()
  };


  // bindings.ru#88 -> __IncompleteArrayField
  let a1 = unsafe {
    ptr33.va_data.as_ptr() 
  };

  // let a2 = unsafe {
  //   ptr33.va_data.as_slice() 
  // };


  a1 as usize
}



/*
  INFO

  #define VARDATA(PTR)        VARDATA_4B(PTR)
  #define VARDATA_4B (PTR)    (((varattrib_4b *) (PTR))->va_4byte.va_data)

*/
