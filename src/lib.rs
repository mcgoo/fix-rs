// Copyright 2016 James Bendig. See the COPYRIGHT file at the top-level
// directory of this distribution.
//
// Licensed under:
//   the MIT license
//     <LICENSE-MIT or https://opensource.org/licenses/MIT>
//   or the Apache License, Version 2.0
//     <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>,
// at your option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(duration_checked_ops)]

#![allow(unknown_lints)]

extern crate chrono;
extern crate mio;
extern crate time;

#[macro_use]
pub mod fixt;
pub mod constant;
#[macro_use]
pub mod field;
pub mod field_type;
pub mod fix;
#[macro_use]
pub mod message;
pub mod rule;

//Dictionary is put last because it needs the above macros.
#[macro_use]
pub mod dictionary;