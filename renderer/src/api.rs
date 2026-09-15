//! Interopt functions and structs between js / rust (wasm).

/// Handle to the Bevy game engine to send messages to it.
#[wasm_bindgen::prelude::wasm_bindgen]
pub struct EngineHandle {
    /// Sender channel to send events to the bevy engine.
    sender: crate::message::Sender,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl EngineHandle {
    /// Spawn an entire city into the engine.
    ///
    /// city must be `[api::server_response::GetOurCityResponse]`
    pub fn load_city(&mut self, city: wasm_bindgen::JsValue) -> Result<(), wasm_bindgen::JsError> {
        let city: api::server_response::GetOurCityResponse = serde_wasm_bindgen::from_value(city)?;
        let message = crate::message::Message::LoadCity { city };
        self.sender.send(message)?;
        Ok(())
    }

    /// Handles an incoming server event.
    pub fn server_event(&mut self, event: wasm_bindgen::JsValue) -> Result<(), wasm_bindgen::JsError> {
        let event: api::server_ws::ServerEvent = serde_wasm_bindgen::from_value(event)?;
        let engine_message = match event {
            api::server_ws::ServerEvent::NewConstruct(construct) => crate::message::Message::NewConstruct {
                construct: construct.construct,
            },
            api::server_ws::ServerEvent::BuildingBuilt(building) => {
                crate::message::Message::ConstructConfirmed { id: building.id }
            }
            api::server_ws::ServerEvent::ConstructStatusUpdated(update) => crate::message::Message::ConstructStatusUpdated {
                id: update.id,
                status: update.status,
            },
            api::server_ws::ServerEvent::ConstructCancelled(cancellation) => {
                crate::message::Message::ConstructCancelled { id: cancellation.id }
            }
        };

        self.sender.send(engine_message)?;
        Ok(())
    }

    /// Start a hovering state for the construct placement.
    ///
    /// If the given kind does not correspond to an existing building kind,
    /// the construct placement state will be reset.
    pub fn start_construct_placement(&mut self, kind: &str) -> Result<(), wasm_bindgen::JsError> {
        let kind = kind.to_string();
        let message = crate::message::Message::StartConstructPlacement { kind };
        self.sender.send(message)?;
        Ok(())
    }

    /// Stop the hovering state for the construct placement.
    pub fn stop_construct_placement(&mut self) -> Result<(), wasm_bindgen::JsError> {
        let message = crate::message::Message::StopConstructPlacement;
        self.sender.send(message)?;
        Ok(())
    }

    /// Ask the engine for the current construction placement,
    /// or None if the engine is not currently handling a construction placement.
    pub async fn ask_construction_position(&mut self) -> Result<ConstructPlacement, wasm_bindgen::JsError> {
        let (sender, receiver) = futures::channel::oneshot::channel();
        let message = crate::message::Message::AskConstructionPlacement { response: sender };
        self.sender.send(message)?;
        let response = receiver.await?.map_err(|e| wasm_bindgen::JsError::new(&e))?;
        Ok(response)
    }
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

#[wasm_bindgen::prelude::wasm_bindgen]
pub struct ConstructPlacement {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[wasm_bindgen::prelude::wasm_bindgen(raw_module = "../lib/bridge.ts")]
extern "C" {
    /// Js function to call when an construct is upvoted
    pub fn vote_for_construct(construct_id: &str, vote: i32);
}
