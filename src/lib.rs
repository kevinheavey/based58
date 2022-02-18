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
#[derive(Debug)]
pub struct Alphabet(AlphabetOriginal);

#[pymethods]
impl Alphabet {
    #[classattr]
    const BITCOIN: Self = Self(*AlphabetOriginal::BITCOIN);
    #[classattr]
    const MONERO: Self = Self(*AlphabetOriginal::MONERO);
    #[classattr]
    const RIPPLE: Self = Self(*AlphabetOriginal::RIPPLE);
    #[classattr]
    const FLICKR: Self = Self(*AlphabetOriginal::FLICKR);
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

#[pyfunction]
pub fn b58decode<'a>(val: &[u8], py: Python<'a>) -> PyResult<&'a PyBytes> {
    let byte_array = decode(val).into_vec().map_err(to_py_value_err)?;
    Ok(PyBytes::new(py, byte_array.as_slice()))
}

#[pyfunction]
pub fn b58encode<'a>(val: &[u8], py: Python<'a>) -> &'a PyBytes {
    let byte_array = encode(val).into_vec();
    PyBytes::new(py, byte_array.as_slice())
}

#[pymodule]
fn based58(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(b58decode, m)?)?;
    m.add_class::<Alphabet>()?;
    Ok(())
}
