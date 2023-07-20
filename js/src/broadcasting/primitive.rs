use arrow2::array::PrimitiveArray;
use geoarrow::algorithm::broadcasting::BroadcastablePrimitive;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BroadcastableFloat(pub(crate) BroadcastablePrimitive<f64>);

#[wasm_bindgen]
impl BroadcastableFloat {
    #[wasm_bindgen(js_name = fromScalar)]
    pub fn from_scalar(value: f64) -> Self {
        Self(BroadcastablePrimitive::Scalar(value))
    }

    #[wasm_bindgen(js_name = fromArray)]
    pub fn from_array(values: Vec<f64>) -> Self {
        Self(BroadcastablePrimitive::Array(PrimitiveArray::from_vec(
            values,
        )))
    }
}

#[wasm_bindgen]
pub struct BroadcastableUint32(pub(crate) BroadcastablePrimitive<u32>);

#[wasm_bindgen]
impl BroadcastableUint32 {
    #[wasm_bindgen(js_name = fromScalar)]
    pub fn from_scalar(value: u32) -> Self {
        Self(BroadcastablePrimitive::Scalar(value))
    }

    #[wasm_bindgen(js_name = fromArray)]
    pub fn from_array(values: Vec<u32>) -> Self {
        Self(BroadcastablePrimitive::Array(PrimitiveArray::from_vec(
            values,
        )))
    }
}
