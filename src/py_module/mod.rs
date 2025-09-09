use pyo3::prelude::*;

mod test_suite;
mod dev;
// mod translate;

// use crate::py_module::translate::*;

#[pymodule]
fn flickers(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // test suite
    let test_suite_mod = PyModule::new(m.py(), "test_suite")?;
    test_suite::init_module(&test_suite_mod)?;
    m.add_submodule(&test_suite_mod)?;

    // dev (the part we want the most !)
    let dev_mod = PyModule::new(m.py(), "dev")?;
    dev::init_module(&dev_mod)?;
    m.add_submodule(&dev_mod)?;

    // add enums to the package
    // m.add_class::<DevType>()?; not needed as long as devtypes do not accept parameters
    // m.add_class::<Afs>()?;
    // m.add_class::<NoiseId>()?;
    // m.add_class::<DevResult>()?;

    Ok(())
}