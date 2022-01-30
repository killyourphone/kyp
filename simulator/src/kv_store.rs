use bricc::prefs::kv_store::KvStore;
use kv::{Bucket, Config, Store};
use serde::{de::DeserializeOwned, Serialize};

pub struct SimKvStore<'a> {
    bucket: Bucket<'a, String, String>,
}

impl<'a> KvStore for SimKvStore<'a> {
    fn get<T: DeserializeOwned>(&mut self, key: String) -> Result<Option<T>, String> {
        match self.bucket.get(key) {
            Ok(opt) => match opt {
                Some(val) => Ok(serde_json::from_str(&val).unwrap()),
                None => Err("Failure".into()),
            },
            Err(_) => Err("Failure".into()),
        }
    }

    fn put(&mut self, key: String, blob: &impl Serialize) -> Result<(), String> {
        match serde_json::to_string(blob) {
            Ok(val) => match self.bucket.set(key, val) {
                Ok(_) => Ok(()),
                Err(err) => Err(err.to_string()),
            },
            Err(err) => Err(err.to_string()),
        }
    }
}

impl<'a> SimKvStore<'a> {
    pub fn new() -> SimKvStore<'a> {
        let cfg = Config::new(".bricc_prefs/");

        // Open the key/value store
        SimKvStore {
            bucket: Store::new(cfg).unwrap().bucket(Some("b")).unwrap(),
        }
    }
}
