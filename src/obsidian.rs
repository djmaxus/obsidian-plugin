use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "obsidian")]
extern "C" {
    pub type Plugin;

    #[wasm_bindgen(structural, method)]
    pub fn addCommand(this: &Plugin, command: JsValue);

    pub type Notice;

    #[wasm_bindgen(constructor)]
    pub fn new(message: &str) -> Notice;

    pub type Editor;

    #[wasm_bindgen(structural, method)]
    pub fn replaceSelection(this: &Editor, replacement: JsString, origin: Option<JsString>);

    #[wasm_bindgen(structural, method)]
    pub fn getSelection(this: &Editor) -> JsString;

    pub type MarkdownView;
}
