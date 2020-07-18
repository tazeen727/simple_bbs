use std::fmt;
use std::convert::TryFrom;
use std::default::Default;
use std::ops::Deref;

use crate::globals;

#[derive(Debug, Clone, Copy)]
pub struct PageNumber(i32);

impl TryFrom<i32> for PageNumber {
    type Error = String;
    fn try_from(value: i32) -> Result<PageNumber, Self::Error> {
        if  value > 0 && value <= globals::MAX_PAGE_NUMBER {
            Ok(PageNumber(value))
        } else {
            Err(format!("PageNumber must be between 1 and {}", globals::MAX_PAGE_NUMBER))
        }
    }
}

impl Default  for PageNumber {
    fn default() -> Self { PageNumber(1) }
}

impl PageNumber {
    pub fn number(&self) -> i32 {
        self.0
    }

    pub fn offset(&self) -> usize {
        let page = self.0;
        let offset = (page - 1) * globals::THREADS_PER_PAGE;
        offset as usize
    }
}

#[derive(Debug, Clone)]
pub struct NonEmptyString(String);

impl TryFrom<String> for NonEmptyString {
    type Error = String;
    fn try_from(value: String) -> Result<NonEmptyString, Self::Error> {
        if value.len() > 0 {
            Ok(NonEmptyString(value))
        } else {
            Err("NonEmptyString must not be empty".to_string())
        }
    }
}

impl TryFrom<&'_ str> for NonEmptyString {
    type Error = String;
    fn try_from(value: &str) -> Result<NonEmptyString, Self::Error> {
        NonEmptyString::try_from(value.to_string())
    }
}

impl Deref for NonEmptyString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<NonEmptyString> for String {
    fn from(value: NonEmptyString) -> Self {
        value.0
    }
}

impl fmt::Display for NonEmptyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
