
#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;
use std::time::Duration;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part,
              ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder,
              Resource, ErrorResponse, remove_json_null_values};



/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View your Google Photos library
    Readonly,

    /// Add to your Google Photos library
    Appendonly,

    /// View and manage your Google Photos library
    Full,

    /// Manage photos added by this app
    ReadonlyAppcreateddata,

    /// Manage and add to shared albums on your behalf
    Sharing,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Readonly => "https://www.googleapis.com/auth/photoslibrary.readonly",
            Scope::Appendonly => "https://www.googleapis.com/auth/photoslibrary.appendonly",
            Scope::Full => "https://www.googleapis.com/auth/photoslibrary",
            Scope::ReadonlyAppcreateddata => {
                "https://www.googleapis.com/auth/photoslibrary.readonly.appcreateddata"
            }
            Scope::Sharing => "https://www.googleapis.com/auth/photoslibrary.sharing",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Readonly
    }
}
