use once_cell::sync::Lazy;
use rand::distributions::{Alphanumeric, DistString};
use crate::infrastructure::security::jwt::keys::Keys;

pub mod api;
pub mod domain;
pub mod infrastructure;
pub mod application;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = Alphanumeric.sample_string(&mut rand::thread_rng(), 60);
    Keys::new(secret.as_bytes())
});