extern crate sgx_types;
extern crate sgx_urts;
extern crate dirs;
extern crate rustc_hex as hex;
use sgx_types::*;
use sgx_urts::SgxEnclave;
use std::io;
use std::io::{Read, Write};
use std::fs;
use std::path;
use std::{ptr, slice};
use hex::ToHex;
static ENCLAVE_FILE: &'static str = "enclave.signed.so";
static ENCLAVE_TOKEN: &'static str = "enclave.token";

extern {
    fn say_something(eid: sgx_enclave_id_t, retval: *mut sgx_status_t,
                     some_string: *const u8, len: usize) -> sgx_status_t;
}

fn init_enclave() -> SgxResult<SgxEnclave> {

    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // Step 1: try to retrieve the launch token saved by last transaction
    //         if there is no token, then create a new one.
    //
    // try to get the token saved in $HOME */
    let mut home_dir = path::PathBuf::new();
    let use_token = match dirs::home_dir() {
        Some(path) => {
            println!("[+] Home dir is {}", path.display());
            home_dir = path;
            true
        },
        None => {
            println!("[-] Cannot get home dir");
            false
        }
    };

    let token_file: path::PathBuf = home_dir.join(ENCLAVE_TOKEN);;
    if use_token == true {
        match fs::File::open(&token_file) {
            Err(_) => {
                println!("[-] Open token file {} error! Will create one.", token_file.as_path().to_str().unwrap());
            },
            Ok(mut f) => {
                println!("[+] Open token file success! ");
                match f.read(&mut launch_token) {
                    Ok(1024) => {
                        println!("[+] Token file valid!");
                    },
                    _ => println!("[+] Token file invalid, will create new token file"),
                }
            }
        }
    }

    // Step 2: call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {secs_attr: sgx_attributes_t { flags:0, xfrm:0}, misc_select:0};
    // let enclave = try!(SgxEnclave::create(ENCLAVE_FILE,
    //                                       debug,
    //                                       &mut launch_token,
    //                                       &mut launch_token_updated,
    //                                       &mut misc_attr));

    let enclave = match SgxEnclave::create(
        ENCLAVE_FILE,
        debug,
        &mut launch_token,
        &mut launch_token_updated,
        &mut misc_attr,
    ) {
        ::core::result::Result::Ok(val) => val,
        ::core::result::Result::Err(err) => {
            return ::core::result::Result::Err(::core::convert::From::from(err))
        }
    };    

    // Step 3: save the launch token if it is updated
    if use_token == true && launch_token_updated != 0 {
        // reopen the file with write capablity
        match fs::File::create(&token_file) {
            Ok(mut f) => {
                match f.write_all(&launch_token) {
                    Ok(()) => println!("[+] Saved updated launch token!"),
                    Err(_) => println!("[-] Failed to save updated launch token!"),
                }
            },
            Err(_) => {
                println!("[-] Failed to save updated enclave token, but doesn't matter");
            },
        }
    }

    Ok(enclave)
}


#[no_mangle]
pub extern "C"
fn ocall_get_quote (output:*mut u8,output_len:u32,value:*mut u8,value_len:usize){
    // println!("104 {:?}",output);
    // let c_list = unsafe{slice::from_raw_parts(in_str, 4)};
    // io::stdout().write(c_list);
    // println!("106 {:?}",c_list);
    // for c in c_list.iter(){
    //     println!("{}",*c as char);
    // }
    let tar = unsafe{slice::from_raw_parts(output, 7).to_vec()};
    let mut hello_string = String::from("");
    for c in tar.iter(){
        hello_string.push(*c as char);
    }
    let mut c= String::from("worileqing");
    let mut tar2 = c.as_bytes().to_vec();
    let mut new_value =tar2.as_mut_ptr();
    unsafe{println!("{:?}",&new_value);}
    unsafe{ptr::copy_nonoverlapping(tar2.as_mut_ptr(), value, tar2.len());}
    // unsafe{value = new_value;}
    let mut value_len = c.len();
    println!("118tar {}", hello_string);
    // let mut valut_len:u32 = 33;
    // let mut value_len:u32 = 33;
    // sgx_status_t::SGX_SUCCESS
}




#[no_mangle]
pub extern "C"
fn ocall_get_service (output:*const u8,output_len:u32,p_len:*mut u32)-> sgx_status_t{
    
    // println!("104 {:?}",output_len);
    println!("ocall_get_service");
    let output_len:usize = 16;
    let output2_len:usize = 10;
    let tar = unsafe{slice::from_raw_parts(output, output_len).to_vec()};
    // let tar2 = unsafe{slice::from_raw_parts(output2, output2_len).to_vec()};
    // io::stdout().write(c_list);
    // println!("106 {:?}",c_list);
    let mut hello_string = String::from("");
    // let mut hello2_string = String::from("");
    
    for c in tar.iter(){
        hello_string.push(*c as char);
    }
    // for c in tar2.iter(){
    //     hello2_string.push(*c as char);
    // }    
    // let tar_input = hello_string.as_ptr() as *const u8;
    println!("tar {:?}", tar);
    // println!("tar {:?}", tar2);    
    println!("tar {:?}", hello_string);
    // println!("tar {:?}", hello2_string);
    
    // unsafe{ptr::copy_nonoverlapping(hello_string.as_bytes().to_vec().as_mut_ptr(), tar_input, hello_string.len());}
    // let mut input_len:u32 = 18;
    sgx_status_t::SGX_SUCCESS
}




fn main() {

    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        },
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        },
    };



    // ocall_get_service(output,&mut output_len,output2,& mut output2_len);
    let input_string = String::from("This is a normal world string passed into Enclave!\n");

    let mut retval = sgx_status_t::SGX_SUCCESS;
    let tar = input_string.as_ptr() as * const u8;
    println!("220 {:?}",tar);
    let result = unsafe {
        say_something(enclave.geteid(),
                      &mut retval,
                      tar,
                      input_string.len())
    };

    match result {
        sgx_status_t::SGX_SUCCESS => {},
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }

    println!("[+] say_something success...");

    enclave.destroy();
}
