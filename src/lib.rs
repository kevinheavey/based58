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

#[pyfunction(alphabet = "Alphabet::BITCOIN")]
pub fn b58decode<'a>(val: &[u8], alphabet: Alphabet, py: Python<'a>) -> PyResult<&'a PyBytes> {
    let byte_array = decode(val)
        .with_alphabet(&alphabet.0)
        .into_vec()
        .map_err(to_py_value_err)?;
    Ok(PyBytes::new(py, byte_array.as_slice()))
}

#[pyfunction(alphabet = "Alphabet::BITCOIN")]
pub fn b58encode<'a>(val: &[u8], alphabet: Alphabet, py: Python<'a>) -> &'a PyBytes {
    let byte_array = encode(val).with_alphabet(&alphabet.0).into_vec();
    PyBytes::new(py, byte_array.as_slice())
}

#[pymodule]
fn based58(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(b58decode, m)?)?;
    m.add_class::<Alphabet>()?;
    Ok(())
}
