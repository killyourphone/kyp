use serde::{de::DeserializeOwned, Serialize};

pub trait KvStore {
    fn get<T: DeserializeOwned>(&mut self, key: String) -> Result<Option<T>, String>;
    fn put(&mut self, key: String, blob: &impl Serialize) -> Result<(), String>;
}
