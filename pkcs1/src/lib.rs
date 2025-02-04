//! Pure Rust implementation of Public-Key Cryptography Standards (PKCS) #1:
//!
//! RSA Cryptography Specifications Version 2.2 ([RFC 8017])
//!
//! ## About
//! This crate supports encoding and decoding RSA private and public keys
//! in either PKCS#1 DER (binary) or PEM (text) formats.
//!
//! PEM encoded RSA private keys begin with:
//!
//! ```text
//! -----BEGIN RSA PRIVATE KEY-----
//! ```
//!
//! PEM encoded RSA public keys begin with:
//!
//! ```text
//! -----BEGIN RSA PUBLIC KEY-----
//! ```
//!
//! Note that PEM-encoded keys must use the [RFC 7468] encoding, which does NOT
//! permit "headers" alongside the data, as used by tools such as OpenSSL.
//!
//! # Minimum Supported Rust Version
//! This crate requires **Rust 1.55** at a minimum.
//!
//! [RFC 7468]: https://tools.ietf.org/html/rfc7468
//! [RFC 8017]: https://tools.ietf.org/html/rfc8017
#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_root_url = "https://docs.rs/pkcs1/0.2.4"
)]
#![forbid(unsafe_code, clippy::unwrap_used)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod error;
mod private_key;
mod public_key;
mod traits;
mod version;

pub use der::{self, asn1::UIntBytes};

pub use self::{
    error::{Error, Result},
    private_key::RsaPrivateKey,
    public_key::RsaPublicKey,
    traits::{DecodeRsaPrivateKey, DecodeRsaPublicKey},
    version::Version,
};

#[cfg(feature = "alloc")]
pub use crate::{
    private_key::{
        document::RsaPrivateKeyDocument, other_prime_info::OtherPrimeInfo, OtherPrimeInfos,
    },
    public_key::document::RsaPublicKeyDocument,
    traits::{EncodeRsaPrivateKey, EncodeRsaPublicKey},
};

#[cfg(feature = "pem")]
#[cfg_attr(docsrs, doc(cfg(feature = "pem")))]
pub use der::pem::{self, LineEnding};
