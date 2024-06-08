pub mod auth;
pub mod image;
pub mod post;
pub mod shared;
pub mod user;

use lazy_static::lazy_static;
use pxid::Factory;

lazy_static! {
    static ref PXID_GENERATOR: Factory = Factory::new().expect("Failed to create Pxid factory");
}
