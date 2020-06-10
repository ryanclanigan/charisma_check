use crate::errors::result::Result;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// Drivers write and read data to/from some arbitrary place
pub trait Driver {
    fn write<'a, T>(self, to_write: &Vec<T>) -> Result<()>
    where
        T: Serialize + Deserialize<'a>;

    fn read<T>(self) -> Result<Vec<T>>
    where
        T: Serialize + DeserializeOwned;
}
