use ::restbl as rstb;
use pyo3::prelude::*;

pyo3::create_exception!(pymsyt, RestblError, pyo3::exceptions::PyException);

/// A Python module implemented in Rust.
#[pymodule]
fn restbl(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ResourceSizeTable>()?;
    Ok(())
}

#[pyclass]
pub struct ResourceSizeTable(rstb::ResourceSizeTable);

#[pymethods]
impl ResourceSizeTable {
    #[staticmethod]
    #[pyo3(text_signature = "(data, /)")]
    pub fn from_binary(data: &[u8]) -> PyResult<Self> {
        let inner = if data.len() > 4
            && u32::from_le_bytes(data[..4].try_into().unwrap()) == 0xFD2FB528
        {
            rstb::ResourceSizeTable::from_binary(zstd::decode_all(data)?)
                .map_err(|e| RestblError::new_err(format!("Failed to parse RSTB file: {:?}", e)))?
        } else {
            rstb::ResourceSizeTable::from_binary(data)
                .map_err(|e| RestblError::new_err(format!("Failed to parse RSTB file: {:?}", e)))?
        };
        Ok(Self(inner))
    }

    #[pyo3(text_signature = "($self)")]
    pub fn to_binary(&self) -> Vec<u8> {
        self.0.to_binary()
    }

    #[pyo3(text_signature = "($self, file, /)")]
    pub fn get_size(&self, file: &str) -> Option<u32> {
        self.0.get(file)
    }

    #[pyo3(text_signature = "($self, file, size, /)")]
    pub fn set_size(&mut self, file: &str, size: u32) {
        self.0.set(file, size);
    }

    #[pyo3(text_signature = "($self, file, /)")]
    pub fn delete_entry(&mut self, file: &str) {
        self.0.remove(file);
    }
}
