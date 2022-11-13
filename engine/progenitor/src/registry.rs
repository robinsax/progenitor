use std::collections::HashMap;

use super::serial::{SerialFormat, SerialError};
use super::errors::InitError;
use super::schema::Type;
use super::store::{Store, StoreError};
use super::store::ext::StoreDriver;
use super::effects::{EffectFn, EffectError};

pub struct Registry {
    effects: HashMap<String, EffectFn>,
    store_drivers: HashMap<String, Box<dyn Fn(&Registry, String) -> Box<dyn StoreDriver>>>,
    serial_formats: HashMap<String, Box<dyn SerialFormat>>,
    config_src: Box<dyn Fn(String) -> Result<String, InitError>>
}

impl Registry {
    pub fn new(
        effect_set: Vec<(&'static str, EffectFn)>,
        store_driver_set: Vec<(&'static str, Box<dyn Fn(&Registry, String) -> Box<dyn StoreDriver>>)>,
        serial_formats_set: Vec<(&'static str, Box<dyn SerialFormat>)>,
        config_src: Box<dyn Fn(String) -> Result<String, InitError>>
    ) -> Self {
        let mut effects = HashMap::with_capacity(effect_set.len());
        for (key, effect) in effect_set {
            effects.insert(key.to_owned(), effect);
        }

        let mut store_drivers = HashMap::with_capacity(store_driver_set.len());
        for (key, factory_fn) in store_driver_set {
            store_drivers.insert(key.to_owned(), factory_fn);
        }

        let mut serial_formats = HashMap::with_capacity(serial_formats_set.len());
        for (key, format) in serial_formats_set {
            serial_formats.insert(key.to_owned(), format);
        }

        Self {
            effects,
            store_drivers,
            serial_formats,
            config_src
        }
    }

    pub fn create_store(&self, schema: Type, driver_name: &str, store_name: String) -> Result<Store, StoreError> {
        let driver_factory = match self.store_drivers.get(driver_name) {
            Some(factory_fn) => factory_fn,
            None => return Err(StoreError::Backend("invalid driver".into()))
        };

        let driver = driver_factory(self, store_name);

        Ok(Store::new(schema, driver))
    }

    pub fn get_serial_format(&self, format_name: &str) -> Result<&Box<dyn SerialFormat>, SerialError> {
        match self.serial_formats.get(format_name) {
            Some(format) => Ok(format),
            None => Err(SerialError::Format("invalid format".into()))
        }
    }

    pub fn get_effect(&self, effect_name: &str) -> Result<EffectFn, EffectError> {
        match self.effects.get(effect_name) {
            Some(effect) => Ok(effect.clone()),
            None => Err(EffectError::Missing(effect_name.to_owned()))
        }
    }

    pub fn get_config(&self, key: impl Into<String>) -> Result<String, InitError> {
        (self.config_src)(key.into())
    }
}
