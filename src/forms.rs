use std::convert::TryFrom;
use rocket::request::FromFormValue;
use rocket::http::RawStr;
use crate::basic_types::*;

impl<'v> FromFormValue<'v> for PageNumber {
    type Error = ();
    fn from_form_value(form_value: &'v RawStr) -> Result<PageNumber, Self::Error> {
        let i = form_value.parse::<i32>().map_err(|_| ())?;
        PageNumber::try_from(i).map_err(|_| ())
    }
}

impl<'v> FromFormValue<'v> for NonEmptyString {
    type Error = ();
    fn from_form_value(form_value: &'v RawStr) -> Result<NonEmptyString, Self::Error> {
        let s = String::from_form_value(form_value).map_err(|_| ())?;
        NonEmptyString::try_from(s.as_str()).map_err(|_| ())
    }
}

pub struct ThreadForm<'a> {
    pub thread_title: &'a str,
    pub poster_name: &'a str,
    pub post_body: &'a str,
}

pub struct PostForm<'a> {
    pub thread_id: i32,
    pub poster_name: &'a str,
    pub post_body: &'a str,
}
