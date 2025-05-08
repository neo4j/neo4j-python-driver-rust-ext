use crate::register_package;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyInt};

pub(super) fn init_module(m: &Bound<PyModule>, name: &str) -> PyResult<()> {
    m.gil_used(false)?;
    register_package(m, name)?;

    m.add_function(wrap_pyfunction!(swap_endian, m)?)?;

    Ok(())
}

#[pyfunction]
pub(super) fn swap_endian<'py>(
    type_size: Bound<'py, PyInt>,
    data: Bound<'py, PyBytes>,
) -> PyResult<Bound<'py, PyBytes>> {
    let py = type_size.py();

    let type_size: usize = match type_size.extract::<usize>() {
        Ok(type_size @ 2) | Ok(type_size @ 4) | Ok(type_size @ 8) => type_size,
        _ => {
            return Err(PyErr::new::<PyValueError, _>(format!(
                "Unsupported type size {}",
                type_size
            )))
        }
    };
    let bytes = &data.as_bytes();
    let len = bytes.len();
    if len % type_size != 0 {
        return Err(PyErr::new::<PyValueError, _>(
            "Data length not a multiple of type_size",
        ));
    }

    PyBytes::new_with(py, bytes.len(), |out| {
        match type_size {
            2 => swap_n::<2>(bytes, out),
            4 => swap_n::<4>(bytes, out),
            8 => swap_n::<8>(bytes, out),
            _ => unreachable!(),
        }
        Ok(())
    })
}

#[inline]
fn swap_n<const N: usize>(src: &[u8], dst: &mut [u8]) {
    // Doesn't technically need to be a function with a const generic, but this
    // allows the compiler to optimize the code better.
    assert_eq!(src.len(), dst.len());
    assert_eq!(src.len() % N, 0);
    for i in (0..src.len()).step_by(N) {
        for j in 0..N {
            dst[i + j] = src[i + N - j - 1];
        }
    }
}
