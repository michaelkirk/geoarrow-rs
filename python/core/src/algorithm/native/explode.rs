use crate::error::PyGeoArrowResult;
use crate::table::GeoTable;
use geoarrow::algorithm::native::Explode;
use pyo3::prelude::*;

/// Explode a table
///
/// Args:
///     input: input table
///
/// Returns:
///     A new table with multi-part geometries exploded to separate rows.
#[pyfunction]
pub fn explode(input: GeoTable) -> PyGeoArrowResult<GeoTable> {
    input.explode()
}

#[pymethods]
impl GeoTable {
    /// Explode a table
    ///
    /// Returns:
    ///     A new table with multi-part geometries exploded to separate rows.
    pub fn explode(&self) -> PyGeoArrowResult<GeoTable> {
        Ok(self.0.explode()?.into())
    }
}
