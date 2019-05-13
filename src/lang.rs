use std::fmt;
use std::borrow::Cow;

use rocket::http::RawStr;
use rocket::request::FromParam;

fn valid_lang(lang: &str) -> bool {
  [
    "auto",
    "javascript",
    "python",
    "rust",
    "go",
    "c",
    "c++"
  ].contains(&lang)
}

pub struct Lang<'a>(Cow<'a, str>);

// impl<'a> Lang<'a> {
//   pub fn new(lang: &str){
//     match valid_lang(lang) {
//       true => Lang(Cow::Owned(lang)),
//       false => Lang(Cow::Owned("auto"))
//     }
//   }
// }

impl<'a> fmt::Display for Lang<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl<'a> FromParam<'a> for Lang<'a> {
  type Error = &'a RawStr;

  fn from_param(param: &'a RawStr) -> Result<Lang<'a>, &'a RawStr> {
    match valid_lang(param) {
      true => Ok(Lang(Cow::Borrowed(param))),
      false => Err(param)
    }
  } 
}