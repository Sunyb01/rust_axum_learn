//! read dir

use std::fs::DirEntry;

use crate::prelude::*;

impl TryFrom<W<&DirEntry>> for String {
    type Error = Error;

    fn try_from(value: W<&DirEntry>) -> Result<Self> {
        Ok(value
            .0
            .path()
            .to_str()
            .map(String::from)
            .ok_or_else(|| Error::Generic(f!("Invalid path: {:?}", value.0)))?)
    }
}
