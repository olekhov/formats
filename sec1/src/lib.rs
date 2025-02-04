//! Pure Rust implementation of [SEC1: Elliptic Curve Cryptography] encoding
//! formats including ASN.1 DER-serialized private keys (also described in
//! [RFC5915]) as well as the `Elliptic-Curve-Point-to-Octet-String` and
//! `Octet-String-to-Elliptic-Curve-Point` encoding algorithms.
//!
//! # Minimum Supported Rust Version
//! This crate requires **Rust 1.55** at a minimum.
//!
//! [SEC1: Elliptic Curve Cryptography]: https://www.secg.org/sec1-v2.pdf
//! [RFC5915]: https://datatracker.ietf.org/doc/html/rfc5915
#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_root_url = "https://docs.rs/sec1/0.2.0-pre"
)]
#![forbid(unsafe_code, clippy::unwrap_used)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod point;

mod error;
mod parameters;
mod private_key;
mod traits;

pub use der;

pub use self::{
    error::{Error, Result},
    parameters::EcParameters,
    point::EncodedPoint,
    private_key::EcPrivateKey,
    traits::DecodeEcPrivateKey,
};

pub use generic_array::typenum::consts;

#[cfg(feature = "alloc")]
pub use crate::{private_key::document::EcPrivateKeyDocument, traits::EncodeEcPrivateKey};

#[cfg(feature = "pem")]
#[cfg_attr(docsrs, doc(cfg(feature = "pem")))]
pub use der::pem::{self, LineEnding};
