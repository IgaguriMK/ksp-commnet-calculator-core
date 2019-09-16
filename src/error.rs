// Copyright 2019 Igaguri.
//
// Licensed under the Apache License, Version 2.0 or the MIT license
// at your option.
//
// Apache License, Version 2.0 <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>
// MIT license <LICENSE-MIT or https://opensource.org/licenses/MIT>

//! Generic error module.

use std::error;
use std::fmt;

/// Generic error type for application binary.
#[derive(Debug)]
pub struct Error(Box<dyn error::Error>);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<E: 'static + error::Error> From<E> for Error {
    fn from(e: E) -> Error {
        Error(Box::new(e))
    }
}

#[derive(Debug)]
pub struct MessageError(String);

impl MessageError {
    pub fn new<S: ToString>(s: S) -> MessageError {
        MessageError(s.to_string())
    }
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl error::Error for MessageError {}
