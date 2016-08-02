// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use git2::Repository;

macro_rules! try_to_opt {
    ($expr: expr) => {
        match $expr {
            Ok(e) => e,
            Err(_) => return None
        }
    };
}

macro_rules! try_opt {
    ($expr: expr) => {
        match $expr {
            Some(e) => e,
            None => return None
        }
    };
}

#[derive(Debug)]
pub struct Version {
    pub version: String,
    pub build: Option<String>,
}

pub fn make() -> Version {
    Version {
        version: ::std::env::var("CARGO_PKG_VERSION").unwrap(),
        build: make_build_number(),
    }
}

impl Version {
    pub fn into_string(self) -> String {
        match self.build {
            Some(b) => self.version + "-" + &*b,
            None => self.version,
        }
    }
}

// build number is based on git sha1 if the current folder is a git repository
fn make_build_number() -> Option<String> {
    // let repo = try_to_opt!(Repository::open("."));
    // let revspec = try_to_opt!(repo.revparse("HEAD"));
    // Some(format!("{}", try_opt!(revspec.from()).id()))
    let repo = Repository::discover(::std::env::var("CARGO_MANIFEST_DIR").unwrap()).expect("not a repository");
    let revspec = repo.revparse("HEAD").expect("revparse fail");
    Some(format!("{}", try_opt!(revspec.from()).id()))
}
