// Copyright 2020 Kyle Mayes
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

//! Handles linking to the `libclang` native library for the `clang-sys` crate.
//!
//! Cargo disallows more than one package in a dependency tree linking to the
//! same native library
//! [using the `package.links` Cargo manifest key](https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key).
//! Before this crate existed, `clang-sys` handled linking to the `libclang`
//! native library and set the `package.links` Cargo manifest key. However, this
//! meant that if a crate ever had more than one version of `clang-sys` in its
//! dependency tree, that crate would fail to compile.
//!
//! This crate was spun out of `clang-sys` in an attempt to address, or at least
//! ameliorate, this problem. With `clang-sys` including this crate as a
//! dependency, multiple versions of `clang-sys` can peacefully coexist in a
//! crate's dependency tree as long as all of the `clang-sys` versions depend on
//! compatible versions of this crate. As this crate does not offer a public
//! API, any future versions can (hopefully) always be safely published as
//! compatible versions (i.e., no major version bumps).
