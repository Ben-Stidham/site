use wasm_bindgen::prelude::*;
pub mod file_system;
pub mod utilities;
pub use file_system::*;
pub use utilities::*;

// This is for external JS calls
#[wasm_bindgen]
extern "C" {
    // Just an example. Not yet implemented
    //pub fn set_output(output: &str);
}

#[wasm_bindgen]
pub fn process_input(key: String, user_info: user_info::UserInfo) -> user_info::UserInfo {
    let mut return_info = user_info.clone();

    return user_info;
}
