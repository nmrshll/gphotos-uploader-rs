#![allow(unused_imports, unused_mut, dead_code)]


#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

// mod cmn;

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

// pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part,
//               ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder,
//               Resource, ErrorResponse, remove_json_null_values};

              /////////////////

use oauth2;
mod scope;
mod hub;



pub const API_VERSION: &str = "v1";
pub const API_BASEPATH: &str = "https://photoslibrary.googleapis.com/";

pub const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
pub const TOKEN_URL: &str = "https://www.googleapis.com/oauth2/v3/token";

pub fn newOauthConfig(client_id: String, client_secret: String) -> oauth2::Config {
    return oauth2::Config::new(client_id, client_secret, AUTH_URL, TOKEN_URL)
        .add_scope("https://www.googleapis.com/auth/calendar")
        .add_scope("https://www.googleapis.com/auth/plus.me");
}
