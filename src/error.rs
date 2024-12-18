// SPDX-License-Identifier: GPL-3.0-only

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct ReasonError<'a, E: Error> {
    msg: &'a str,
    source: E,
}

impl<E: Error> Display for ReasonError<'_, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg)?;
        f.write_str(": ")?;
        Display::fmt(&self.source, f)
    }
}

impl<E: Error> Error for ReasonError<'_, E> {}

pub fn err_reason<E: Error>(msg: &str, source: E) -> ReasonError<E> {
    ReasonError {
        msg,
        source,
    }
}