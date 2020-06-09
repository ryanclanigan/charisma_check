use super::driver::Driver;
use crate::errors::error::Error;
use crate::errors::result::Result;
use csv::{Reader, Writer};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// A driver which specifically deals with csv files
#[derive(Copy, Clone)]
pub struct CsvDriver<'a> {
    path: &'a Path,
}

impl<'b> CsvDriver<'b> {
    pub fn new(path: &'b Path) -> Result<Self> {
        match path.to_str() {
            None => Err(Error::InvalidFile("Invalid FileName".to_string())),
            _ => Ok(CsvDriver { path }),
        }
    }
}

impl<'b> Driver for CsvDriver<'b> {
    fn write<'a, T>(self, to_write: Vec<T>) -> Result<()>
    where
        T: Serialize + Deserialize<'a>,
    {
        let mut writer = match Writer::from_path(self.path) {
            Ok(s) => s,
            Err(e) => return Err(Error::WriteError(e.to_string())),
        };

        for record in to_write {
            match writer.serialize(record) {
                Err(e) => return Err(Error::WriteError(e.to_string())),
                _ => (),
            };
        }

        match writer.flush() {
            Err(e) => Err(Error::WriteError(e.to_string())),
            Ok(_) => Ok(()),
        }
    }

    fn read<T>(self) -> Result<Vec<T>>
    where
        T: Serialize + DeserializeOwned,
    {
        let mut reader = match Reader::from_path(self.path) {
            Ok(s) => s,
            Err(e) => return Err(Error::ReadError(e.to_string())),
        };

        let mut results: Vec<T> = Vec::new();
        for record in reader.deserialize() {
            let result: T = match record {
                Ok(t) => t,
                Err(e) => return Err(Error::ReadError(e.to_string())),
            };
            results.push(result);
        }

        Ok(results)
    }
}
