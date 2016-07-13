// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;

/// A set of errors that can occur parsing HTTP streams.
pub enum Error {
    /// An invalid `Method`, such as `GE,T`.
    Method,
    /// An invalid `RequestUri`, such as `exam ple.domain`.
    Uri(url::ParseError),
    /// An invalid `HttpVersion`, such as `HTP/1.1`
    Version,
    /// An invalid `Header`.
    Header,
    /// A message head is too large to be reasonable.
    TooLarge,
    /// A message reached EOF, but is not complete.
    Incomplete,
    /// An invalid `Status`, such as `1337 ELITE`.
    Status,
    /// A timeout occurred waiting for an IO event.
    Timeout,
    /// An `io::Error` that occurred while trying to read or write to a network stream.
    Io(IoError),
    /// An error from a SSL library.
    Ssl(Box<StdError + Send + Sync>),
    /// Parsing a field as string failed
    Utf8(Utf8Error),

    #[doc(hidden)]
    __Nonexhaustive(Void)
}
