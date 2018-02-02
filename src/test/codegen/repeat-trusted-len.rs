// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags: -O
// ignore-tidy-linelength

#![crate_type = "lib"]

use std::iter;

// CHECK-LABEL: @repeat_take_collect
#[no_mangle]
pub fn repeat_take_collect() -> Vec<u8> {
// CHECK: call void @llvm.memset.p0i8
    iter::repeat(42).take(100000).collect()
}

// CHECK-LABEL: @range_from_take_collect
#[no_mangle]
pub fn range_from_take_collect() -> Vec<u8> {
// CHECK: %broadcast.splatinsert = insertelement <{{[0-9]+}} x i8> undef, i8 %{{.*}}, i32 0
// CHECK: %broadcast.splat = shufflevector <[[WIDTH:[0-9]+]] x i8> %broadcast.splatinsert, <[[WIDTH]] x i8> undef, <[[WIDTH]] x i32> zeroinitializer
    (0..).take(100000).collect()
}
