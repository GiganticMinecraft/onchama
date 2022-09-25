use derive_new::new;
use once_cell::sync::Lazy;
use poise::serenity_prelude::ChannelId;
use std::sync::{Arc, Mutex};

#[derive(new, Default)]
pub struct DataModel<T: Default + Clone>(#[new(default)] Lazy<Arc<Mutex<Option<T>>>>);

impl<T: Default + Clone> DataModel<T> {
    pub fn save(&self, value: T) {
        let mut lock = self.0.lock().unwrap();
        *lock = Some(value)
    }

    pub fn clear(&self) {
        let mut lock = self.0.lock().unwrap();
        *lock = None
    }

    pub fn get(&self) -> Option<T> {
        self.0.lock().unwrap().clone()
    }
}

#[derive(new)]
pub struct Data {
    #[new(default)]
    pub text_ch: DataModel<ChannelId>,
}
