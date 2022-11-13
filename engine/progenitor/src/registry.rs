use std::collections::HashMap;

use super::schema::Type;
use super::store::{Store, StoreError};
use super::store::ext::StoreDriver;
use super::effects::{EffectFn, EffectError};

pub struct Registry {
    effects: HashMap<String, EffectFn>,
    store_drivers: HashMap<String, Box<dyn Fn(&Registry, String) -> Box<dyn StoreDriver>>>,
    config: HashMap<String, String>
}

impl Registry {
    fn new(
        effect_set: Vec<(String, EffectFn)>,
        store_driver_set: Vec<(String, Box<dyn Fn(&Registry, String) -> Box<dyn StoreDriver>>)>,
        config_set: Vec<(String, String)>
    ) -> Self {
        let mut effects = HashMap::with_capacity(effect_set.len());
        for (key, effect) in effect_set {
            effects.insert(key, effect);
        }

        let mut store_drivers = HashMap::with_capacity(store_driver_set.len());
        for (key, factory_fn) in store_driver_set {
            store_drivers.insert(key, factory_fn);
        }

        let mut config = HashMap::with_capacity(config_set.len());
        for (key, value) in config_set {
            config.insert(key, value);
        }

        Self {
            effects,
            store_drivers,
            config
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

    pub fn get_effect(&self, effect_name: &str) -> Result<EffectFn, EffectError> {
        match self.effects.get(effect_name) {
            Some(effect) => Ok(effect.clone()),
            None => Err(EffectError::Missing(effect_name.to_owned()))
        }
    }
}
