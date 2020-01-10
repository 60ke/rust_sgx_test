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


extern crate rustc_hex;
extern crate http_req;
use http_req::request;
use std::str::FromStr;
use std::ptr;
use sgx_types::*;
// use std::string::String;
// use std::vec::Vec;
use std::io::{self, Write};
use std::slice;
use rustc_hex::ToHex;
use std::{boxed::Box, string::{String, ToString}, vec::Vec};

extern "C" {
    pub fn ocall_get_quote(output:*const u8,state_len:u32,value:*mut u8,p_quote_len: u32);
}



#[no_mangle]
extern "C" {
    pub fn ocall_get_ias_socket (
        output:*const u8,state_size:*mut usize);
    }

// extern "C" {
//     pub fn ocall_get_service(p_spid:*const sgx_spid_t)-> sgx_status_t;
// }




static EMPTY: [u8; 1] = [0];

/// This trait provides an interface into `C` like pointers.
/// in Rust if you try to get a pointer to an empty vector you'll get:
/// 0x0000000000000001 OR 0x0000000000000000, although bear in mind this *isn't* officially defined.
/// this behavior is UB in C's `malloc`, passing an invalid pointer with size 0 to `malloc` is implementation defined.
/// in the case of Intel's + GCC what we observed is a Segmentation Fault.
/// this is why if the vec/slice is empty we use this trait to pass a pointer to a stack allocated static `[0]` array.
/// this will make the pointer valid, and when the len is zero
/// `malloc` won't allocate anything but also won't produce a SegFault
pub trait SliceCPtr {
    /// The Target for the trait.
    /// this trait can't be generic because it should only be implemented once per type
    /// (See [Associated Types][https://doc.rust-lang.org/rust-by-example/generics/assoc_items/types.html])
    type Target;
    /// This function is what will produce a valid C pointer to the target
    /// even if the target is 0 sized (and rust will produce a C *invalid* pointer for it )
    fn as_c_ptr(&self) -> *const Self::Target;
}

impl<T> SliceCPtr for [T] {
    type Target = T;
    fn as_c_ptr(&self) -> *const Self::Target {
        if self.is_empty() {
            EMPTY.as_ptr() as *const _
        } else {
            self.as_ptr()
        }
    }
}

impl SliceCPtr for str {
    type Target = u8;
    fn as_c_ptr(&self) -> *const Self::Target {
        if self.is_empty() {
            EMPTY.as_ptr() as *const _
        } else {
            self.as_ptr()
        }
    }
}


#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {
    let mut b= String::from("1233890");
    let mut c= String::from("ddddd");
    let mut tar = b.as_bytes().to_vec();
    let mut tar2 = c.as_bytes().to_vec();

    let mut output =tar.as_mut_ptr();
    let mut value =tar2.as_mut_ptr();
    
    let mut output_len = b.len();
    let mut value_len = c.len();
    let mut c:u32 = 16;
    // let mut tar_input = String::from("okokok").as_bytes().to_vec().as_mut_ptr();
    // let input_len = &mut c as *mut u32;
    
    let mut tar_in = vec![0u8; 16];
    let slice = tar_in.as_mut_slice();
    let tar_input = slice.as_mut_ptr() as *mut u8;
    let tar_input_len = tar_in.len();
    let mut state_len:u32 = 16;
    let mut p_len:u32 = 10;
    let p_quote_len = &mut p_len as *mut u32;
    let mut retval = sgx_status_t::SGX_SUCCESS;
    let mut rt : sgx_status_t = sgx_status_t::SGX_SUCCESS;
    unsafe{ptr::copy_nonoverlapping(tar.as_c_ptr(), output, tar.len());}
    let result = unsafe {ocall_get_quote(output,state_len,value,state_len)};
    let tar2 = unsafe{slice::from_raw_parts(value, 10).to_vec()};
    // let mut c= String::from("ddddd");
    // let mut tar2 = c.as_bytes().to_vec();
    // let mut value =tar2.as_mut_ptr();
    let mut hello_string = String::from("");
    for c in tar2.iter(){
        hello_string.push(*c as char);
    }
    println!("enclave 149{:?}",hello_string);    
    println!("enclave 136");

    sgx_status_t::SGX_SUCCESS
}