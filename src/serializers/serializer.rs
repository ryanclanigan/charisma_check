use crate::errors::result::Result;

/// Serializers sit on top of drivers, taking a given serializable
/// thing and writing/reading it via the underlying driver
pub trait Serializer<T> {
    fn write(&self, to_write: Vec<T>) -> Result<()>;
    fn read(&self) -> Result<Vec<T>>;
}
