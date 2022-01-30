use bricc::prefs::kv_store::KvStore;
use embedded_svc::storage::Storage;
use esp_idf_svc::nvs::{EspDefaultNvs, EspNvs};
use esp_idf_svc::nvs_storage::EspNvsStorage;
use serde::de::DeserializeOwned;
use std::io::Error;
use std::sync::Arc;

pub struct EspKvStore {
    nvs: EspNvsStorage,
}

impl KvStore for EspKvStore {
    fn get<T: DeserializeOwned>(&mut self, key: String) -> Result<Option<T>, String> {
        match self.nvs.get(key) {
            Ok(val) => Ok(val),
            Err(err) => Err(err.to_string()),
        }
    }

    fn put(&mut self, key: String, blob: &impl serde::Serialize) -> Result<(), String> {
        match self.nvs.put(key, blob) {
            Ok(val) => {
                if val {
                    Ok(())
                } else {
                    Err("Failed to save?".into())
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }
}

impl EspKvStore {
    pub fn new(default_nvs: Arc<EspDefaultNvs>) -> EspKvStore {
        EspKvStore {
            nvs: match EspNvsStorage::new_default(default_nvs, "kyp", true) {
                Ok(nvs) => nvs,
                Err(err) => {
                    println!("Couldn't open nvs {}", err);
                    panic!()
                }
            },
        }
    }
}
