/// Wasm compatible version of the building.
///
/// This is the type that JS shall give us.
#[wasm_bindgen::prelude::wasm_bindgen]
pub struct Building {
    name: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

/// Wasm compatible version of the city.
///
/// This is the type that JS shall give us.
#[wasm_bindgen::prelude::wasm_bindgen]
pub struct City {
    buildings: Vec<Building>,
}
