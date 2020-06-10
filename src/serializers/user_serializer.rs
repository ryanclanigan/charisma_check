use super::drivers::csv_driver::CsvDriver;
use super::drivers::driver::Driver;
use super::serializer::Serializer;
use crate::datums::user_record::UserRecord;
use crate::errors::result::Result;
use std::path::Path;

/// A serializer for users and their metadata
pub struct UserSerializer<'a> {
    driver: CsvDriver<'a>,
}

impl<'a> UserSerializer<'a> {
    pub fn new(path: &'a Path) -> Result<Self> {
        match CsvDriver::new(path) {
            Ok(driver) => Ok(UserSerializer { driver }),
            Err(e) => Err(e),
        }
    }
}

impl<'a> Serializer<UserRecord> for UserSerializer<'a> {
    fn write(&self, records: &Vec<UserRecord>) -> Result<()> {
        self.driver.write(records)
    }

    fn read(&self) -> Result<Vec<UserRecord>> {
        self.driver.read()
    }
}
