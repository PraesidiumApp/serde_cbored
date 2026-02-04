//! A CBOR backend for the serde framework

#[cfg(not(any(feature = "ser", feature = "de")))]
compile_error!("You must enable either \"ser\" or \"de\" features");

pub mod error;

#[cfg(feature = "de")]
pub mod de;

#[cfg(feature = "ser")]
pub mod ser;
