use std::error::Error;

// use bs58::decode::Error;
use bs58::{decode, encode, Alphabet as AlphabetOriginal};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

fn to_py_value_err<T: Error>(err: T) -> PyErr {
    PyValueError::new_err(err.to_string())
}

#[pyclass]
#[derive(Debug, Clone)]
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

#[pyfunction(alphabet = "Alphabet::BITCOIN")]
pub fn b58decode<'a>(val: &[u8], alphabet: Alphabet, py: Python<'a>) -> PyResult<&'a PyBytes> {
    let byte_vec = decode(val)
        .with_alphabet(&alphabet.0)
        .into_vec()
        .map_err(to_py_value_err)?;
    Ok(byte_vec_to_pybytes(&byte_vec, py))
}

#[pyfunction(alphabet = "Alphabet::BITCOIN")]
pub fn b58encode<'a>(val: &[u8], alphabet: Alphabet, py: Python<'a>) -> &'a PyBytes {
    let byte_vec = encode(val).with_alphabet(&alphabet.0).into_vec();
    byte_vec_to_pybytes(&byte_vec, py)
}

#[pyfunction(alphabet = "Alphabet::BITCOIN", expected_ver = "None")]
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

#[pyfunction(alphabet = "Alphabet::BITCOIN", expected_ver = "None")]
pub fn b58encode_check<'a>(
    val: &[u8],
    alphabet: Alphabet,
    py: Python<'a>,
    expected_ver: Option<u8>,
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
