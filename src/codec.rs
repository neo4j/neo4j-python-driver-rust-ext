mod packstream;

use pyo3::prelude::*;

use crate::register_package;

pub(super) fn init_module(m: &Bound<PyModule>, name: &str) -> PyResult<()> {
    let py = m.py();

    m.gil_used(false)?;
    register_package(m, name)?;

    let mod_packstream = PyModule::new(py, "packstream")?;
    m.add_submodule(&mod_packstream)?;
    packstream::init_module(&mod_packstream, format!("{name}.packstream").as_str())?;

    Ok(())
}
