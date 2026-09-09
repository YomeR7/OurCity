/// Handle to the Bevy game engine to send messages to it.
#[wasm_bindgen::prelude::wasm_bindgen]
pub struct EngineHandle {
    /// Sender channel to send events to the bevy engine.
    sender: crate::message::Sender,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl EngineHandle {
    pub fn load_city(&mut self, city: crate::api::building::City) {}
    pub fn building_confirmed(&mut self, building: crate::api::building::Building) {}
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn init_engine(canvas_id: &str) -> EngineHandle {
    /* Create a panic hook to get nice error messages */
    console_error_panic_hook::set_once();

    /* Create the channel between the engine and the handle to send messages */
    let (sender, receiver) = std::sync::mpsc::channel();

    /* Init and start the bevy engine */
    crate::engine::run_bevy_engine(canvas_id, receiver);

    EngineHandle { sender }
}
