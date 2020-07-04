// Copyright 2016 Kyle Mayes
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Finds `libclang` static or dynamic libraries and links to them.
//!
//! # Environment Variables
//!
//! This build script can make use of several environment variables to help it
//! find the required static or dynamic libraries.
//!
//! * `LLVM_CONFIG_PATH` - provides a path to an `llvm-config` executable
//! * `LIBCLANG_PATH` - provides a path to a directory containing a `libclang`
//!    shared library or a path to a specific `libclang` shared library
//! * `LIBCLANG_STATIC_PATH` - provides a path to a directory containing LLVM
//!    and Clang static libraries

#![allow(unused_attributes)]

extern crate glob;

use std::path::Path;

#[path = "../build/common.rs"]
pub mod common;
#[path = "../build/dynamic.rs"]
pub mod dynamic;
#[path = "../build/static.rs"]
pub mod static_;

/// Finds and links to the required libraries.
fn main() {
    if cfg!(feature = "runtime") {
        return;
    }

    if cfg!(feature = "static") {
        static_::link();
    } else {
        dynamic::link();
    }

    if let Some(output) = common::run_llvm_config(&["--includedir"]) {
        let directory = Path::new(output.trim_end());
        println!("cargo:include={}", directory.display());
    }
}
