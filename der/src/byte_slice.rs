//! Common handling for types backed by byte slices with enforcement of a
//! library-level length limitation i.e. `Length::max()`.

use crate::{
    str_slice::StrSlice, DecodeValue, Decoder, EncodeValue, Encoder, Error, ErrorKind, Length,
    Result,
};
use core::convert::TryFrom;

/// Byte slice newtype which respects the `Length::max()` limit.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct ByteSlice<'a> {
    /// Inner value
    inner: &'a [u8],

    /// Precomputed `Length` (avoids possible panicking conversions)
    length: Length,
}

impl<'a> ByteSlice<'a> {
    /// Create a new [`ByteSlice`], ensuring that the provided `slice` value
    /// is shorter than `Length::max()`.
    pub fn new(slice: &'a [u8]) -> Result<Self> {
        Ok(Self {
            inner: slice,
            length: Length::try_from(slice.len())?,
        })
    }

    /// Borrow the inner byte slice
    pub fn as_bytes(&self) -> &'a [u8] {
        self.inner
    }

    /// Get the [`Length`] of this [`ByteSlice`]
    pub fn len(self) -> Length {
        self.length
    }

    /// Is this [`ByteSlice`] empty?
    pub fn is_empty(self) -> bool {
        self.len() == Length::ZERO
    }
}

impl AsRef<[u8]> for ByteSlice<'_> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a> DecodeValue<'a> for ByteSlice<'a> {
    fn decode_value(decoder: &mut Decoder<'a>, length: Length) -> Result<Self> {
        decoder
            .bytes(length)
            .map_err(|_| decoder.error(ErrorKind::Truncated))
            .and_then(Self::new)
    }
}

impl<'a> EncodeValue for ByteSlice<'a> {
    fn value_len(&self) -> Result<Length> {
        Ok(self.length)
    }

    fn encode_value(&self, encoder: &mut Encoder<'_>) -> Result<()> {
        encoder.bytes(self.as_ref())
    }
}

impl Default for ByteSlice<'_> {
    fn default() -> Self {
        Self {
            inner: &[],
            length: Length::ZERO,
        }
    }
}

impl<'a> From<&'a [u8; 1]> for ByteSlice<'a> {
    fn from(byte: &'a [u8; 1]) -> ByteSlice<'a> {
        Self {
            inner: byte,
            length: Length::ONE,
        }
    }
}

impl<'a> From<StrSlice<'a>> for ByteSlice<'a> {
    fn from(s: StrSlice<'a>) -> ByteSlice<'a> {
        let bytes = s.as_bytes();
        debug_assert_eq!(bytes.len(), usize::try_from(s.length).expect("overflow"));

        ByteSlice {
            inner: bytes,
            length: s.length,
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for ByteSlice<'a> {
    type Error = Error;

    fn try_from(slice: &'a [u8]) -> Result<Self> {
        Self::new(slice)
    }
}
