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

#[cfg(feature = "runtime")]
use std::path::Path;

/// Copy the file from the supplied source to the supplied destination.
#[cfg(feature = "runtime")]
fn copy(source: &str, destination: &Path) {
    use std::fs::File;
    use std::io::{Read, Write};

    let mut string = String::new();
    File::open(source)
        .unwrap()
        .read_to_string(&mut string)
        .unwrap();
    File::create(destination)
        .unwrap()
        .write_all(string.as_bytes())
        .unwrap();
}

/// Generates the finding and linking code so that it may be used at runtime.
#[cfg(feature = "runtime")]
fn main() {
    use std::env;

    if cfg!(feature = "static") {
        panic!("`runtime` and `static` features can't be combined");
    }

    let out = env::var("OUT_DIR").unwrap();
    copy("../build/common.rs", &Path::new(&out).join("common.rs"));
    copy("../build/dynamic.rs", &Path::new(&out).join("dynamic.rs"));
}

#[cfg(not(feature = "runtime"))]
fn main() { }
