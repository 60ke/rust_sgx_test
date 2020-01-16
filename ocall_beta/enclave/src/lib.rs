// Copyright (C) 2017-2019 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
use sgx_types::*;
use std::io::{self, Write};
use std::slice;
use std::{string::{String}};

extern "C" {
    pub fn ocall_get_quote(output:*const u8,state_len:u32,value:*mut u8,p_quote_len:*mut u8);
}


#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {


// ecall

    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

// ocall
    // 构建encalve请求key的指针
    let mut e_key= String::from("enclave_request");
    let mut e_key_vec = e_key.as_bytes().to_vec();
    let mut e_key_ptr =e_key_vec.as_mut_ptr();

    // 构建encalve接收value的指针
    let mut e_value= vec![0u8; 1000];
    let mut value =e_value.as_mut_ptr();
    
    // 定义接收value的变量    
    let mut value_len:u8 = 0;

    let result = unsafe {ocall_get_quote(e_key_ptr,e_key.len() as u32,value,&mut value_len)};
    let app_response = unsafe{slice::from_raw_parts(value, value_len as usize).to_vec()};
    let mut response_string = String::from("");
    for c in app_response.iter(){
       response_string.push(*c as char);
    }
    println!("app 返回到encalve的字符串{:?}",response_string);    

    sgx_status_t::SGX_SUCCESS
}