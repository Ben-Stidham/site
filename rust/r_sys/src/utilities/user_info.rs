use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct UserInfo {
    input_buffer: Vec<char>,
    history: Vec<String>,
    output: Vec<String>,
    current_path: Vec<String>,
}

impl UserInfo {
    pub fn new() -> Self {
        return Self {
            input_buffer: Vec::new(),
            history: Vec::new(),
            output: Vec::new(),
            current_path: Vec::new(),
        };
    }

    pub fn append_buffer(&mut self, input: char) {
        self.input_buffer.push(input);
    }

    pub fn purge_buffer(&mut self) {
        self.input_buffer = Vec::new();
    }

    pub fn read_buffer(&self) -> Vec<char> {
        return self.input_buffer.clone();
    }

    pub fn get_history(&self) -> Vec<String> {
        return self.history.clone();
    }

    pub fn get_output(&self) -> Vec<String> {
        return self.output.clone();
    }

    pub fn get_current_path(&self) -> Vec<String> {
        return self.current_path.clone();
    }

    pub fn set_history(&mut self, history: Vec<String>) {
        self.history = history;
    }

    pub fn set_output(&mut self, output: Vec<String>) {
        self.output = output;
    }

    pub fn set_current_path(&mut self, current_path: Vec<String>) {
        self.current_path = current_path;
    }
}
