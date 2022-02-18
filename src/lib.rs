use std::error::Error;

// use bs58::decode::Error;
use bs58::{decode, encode, Alphabet as AlphabetOriginal};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

fn to_py_value_err<T: Error>(err: T) -> PyErr {
    PyValueError::new_err(err.to_string())
}

/// A collection of 58 ASCII characters used to encode data.
///
/// Args:
///      base (bytes): The 58 ASCII characters with which to create the alphabet.
///      
/// Example:
///     >>> from based58 import Alphabet, b58decode, b58encode
///     >>> alpha = Alphabet(b" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXY")
///     >>> decoded = b58decode(b"he11owor1d", alphabet=Alphabet.RIPPLE)
///     >>> decoded
///     b'`e\xe7\x9b\xba/x'
///     >>> b58encode(decoded, alphabet=alpha)
///     b'#ERRN)N RD'
#[pyclass]
#[derive(Debug, Clone)]
#[pyo3(text_signature = "(base)")]
pub struct Alphabet(pub AlphabetOriginal);

#[pymethods]
impl Alphabet {
    /// Bitcoin's alphabet as defined in their Base58Check encoding.
    ///
    /// See <https://en.bitcoin.it/wiki/Base58Check_encoding#Base58_symbol_chart>
    #[classattr]
    const BITCOIN: Self = Self(*AlphabetOriginal::BITCOIN);
    /// Monero's alphabet as defined in this forum post.
    ///
    /// See <https://forum.getmonero.org/4/academic-and-technical/221/creating-a-standard-for-physical-coins>
    #[classattr]
    const MONERO: Self = Self(*AlphabetOriginal::MONERO);
    /// Ripple's alphabet as defined in their wiki.
    ///
    /// See <https://wiki.ripple.com/Encodings>
    #[classattr]
    const RIPPLE: Self = Self(*AlphabetOriginal::RIPPLE);
    /// Flickr's alphabet for creating short urls from photo ids.
    ///
    /// See <https://www.flickr.com/groups/api/discuss/72157616713786392/>
    #[classattr]
    const FLICKR: Self = Self(*AlphabetOriginal::FLICKR);
    /// The default alphabet used if none is given. Currently is the
    /// [`BITCOIN`](Self::BITCOIN) alphabet.
    #[classattr]
    const DEFAULT: Self = Self(*AlphabetOriginal::DEFAULT);

    #[new]
    pub fn new(base: &[u8]) -> PyResult<Self> {
        let sized_base: &[u8; 58] = base.try_into().map_err(|_| {
            PyValueError::new_err(format!(
                "Expected a bytes of length {} but received length {}",
                58,
                base.len()
            ))
        })?;
        let underlying = AlphabetOriginal::new(sized_base).map_err(to_py_value_err)?;
        Ok(Self(underlying))
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

fn byte_vec_to_pybytes<'a>(v: &Vec<u8>, py: Python<'a>) -> &'a PyBytes {
    PyBytes::new(py, v.as_slice())
}

/// Decode a base-58 value.
///
/// Args:
///     val (bytes): The bytes to decode.
///     alphabet (Alphabet, optional): The encoding alphabet. Defaults to :attr:`Alphabet.BITCOIN`.
///     
/// Returns:
///     bytes: The decoded value.
///     
/// Example:
///     >>> from based58 import b58decode, Alphabet
///     >>> b58decode(b"he11owor1d")
///     b'\x040^+$s\xf0X'
///     >>> b58decode(b"he11owor1d", Alphabet.RIPPLE)
///     b'`e\xe7\x9b\xba/x'
///
#[pyfunction(alphabet = "Alphabet::BITCOIN")]
#[pyo3(text_signature = "(val, alphabet)")]
pub fn b58decode<'a>(val: &[u8], alphabet: Alphabet, py: Python<'a>) -> PyResult<&'a PyBytes> {
    let byte_vec = decode(val)
        .with_alphabet(&alphabet.0)
        .into_vec()
        .map_err(to_py_value_err)?;
    Ok(byte_vec_to_pybytes(&byte_vec, py))
}

/// Encode bytes into base-58.
///
/// Args:
///     val (bytes): The bytes to encode.
///     alphabet (Alphabet, optional): The encoding alphabet. Defaults to :attr:`Alphabet.BITCOIN`.
///     
/// Returns:
///     bytes: The encoded value.
///     
/// Example:
///     >>> from based58 import b58encode, Alphabet
///     >>> b58encode(b"\x040^+$s\xf0X")
///     b'he11owor1d'
///     >>> b58encode(b'`e\xe7\x9b\xba/x', Alphabet.RIPPLE)
///     b'he11owor1d'
///
#[pyfunction(alphabet = "Alphabet::BITCOIN")]
#[pyo3(text_signature = "(val, alphabet)")]
pub fn b58encode<'a>(val: &[u8], alphabet: Alphabet, py: Python<'a>) -> &'a PyBytes {
    let byte_vec = encode(val).with_alphabet(&alphabet.0).into_vec();
    byte_vec_to_pybytes(&byte_vec, py)
}

/// Decode and check checksum using the
/// `Base58Check <https://en.bitcoin.it/wiki/Base58Check_encoding>`_ algorithm.
///
/// Args:
///     val (bytes): The bytes to decode.
///     alphabet (Alphabet, optional): The encoding alphabet. Defaults to :attr:`Alphabet.BITCOIN`.
///     expected_ver (int, optional):  If provided, the version byte will be used in verification. Defaults to None.
///
/// Returns:
///     bytes: The decoded value.
///
/// Example:
///     >>> from based58 import b58decode_check
///     >>> b58decode_check(b"PWEu9GGN")
///     b'-1'
///
#[pyfunction(alphabet = "Alphabet::BITCOIN", expected_ver = "None")]
#[pyo3(text_signature = "(val, alphabet, expected_ver = None)")]
pub fn b58decode_check<'a>(
    val: &[u8],
    alphabet: Alphabet,
    expected_ver: Option<u8>,
    py: Python<'a>,
) -> PyResult<&'a PyBytes> {
    let byte_vec = decode(val)
        .with_alphabet(&alphabet.0)
        .with_check(expected_ver)
        .into_vec()
        .map_err(to_py_value_err)?;
    Ok(byte_vec_to_pybytes(&byte_vec, py))
}

/// Encode and check checksum using the
/// `Base58Check <https://en.bitcoin.it/wiki/Base58Check_encoding>`_ algorithm.
///
/// Args:
///     val (bytes): The bytes to encode.
///     alphabet (Alphabet, optional): The encoding alphabet. Defaults to :attr:`Alphabet.BITCOIN`.
///     expected_ver (int, optional):  If provided, the version byte will be used in verification. Defaults to None.
///
/// Returns:
///     bytes: The encoded value.
///
/// Example:
///     >>> from based58 import b58encode_check
///     >>> b58encode_check(b"`e\xe7\x9b\xba/x")
///     b'QuT57JNzzWTu7mW'
///
#[pyfunction(alphabet = "Alphabet::BITCOIN", expected_ver = "None")]
#[pyo3(text_signature = "(val, alphabet, expected_ver = None)")]
pub fn b58encode_check<'a>(
    val: &[u8],
    alphabet: Alphabet,
    expected_ver: Option<u8>,
    py: Python<'a>,
) -> &'a PyBytes {
    let builder = encode(val).with_alphabet(&alphabet.0);
    let with_check = {
        if let Some(ver) = expected_ver {
            builder.with_check_version(ver)
        } else {
            builder.with_check()
        }
    };
    let byte_vec = with_check.into_vec();
    byte_vec_to_pybytes(&byte_vec, py)
}

#[pymodule]
fn based58(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(b58decode, m)?)?;
    m.add_function(wrap_pyfunction!(b58encode, m)?)?;
    m.add_function(wrap_pyfunction!(b58decode_check, m)?)?;
    m.add_function(wrap_pyfunction!(b58encode_check, m)?)?;
    m.add_class::<Alphabet>()?;
    Ok(())
}
