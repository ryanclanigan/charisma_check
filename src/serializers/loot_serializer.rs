use super::drivers::csv_driver::CsvDriver;
use super::drivers::driver::Driver;
use super::serializer::Serializer;
use crate::datums::loot_record::LootRecord;
use crate::errors::result::Result;
use std::path::Path;

pub struct LootSerializer<'a> {
    driver: CsvDriver<'a>,
}

impl<'a> LootSerializer<'a> {
    pub fn new(path: &'a Path) -> Result<Self> {
        match CsvDriver::new(path) {
            Ok(driver) => Ok(LootSerializer { driver }),
            Err(e) => Err(e),
        }
    }
}

impl<'a> Serializer<LootRecord> for LootSerializer<'a> {
    fn write(&self, to_write: &Vec<LootRecord>) -> Result<()> {
        Ok(())
    }

    fn read(&self) -> Result<Vec<LootRecord>> {
        self.driver.read()
    }
}
