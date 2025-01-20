//! This module contains the bindings to the Obsidian API.
//! https://github.com/obsidianmd/obsidian-api

use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "obsidian")]
extern "C" {
    pub type Plugin;

    #[wasm_bindgen(structural, method)]
    pub fn addCommand(this: &Plugin, command: JsValue);
}

#[wasm_bindgen(module = "obsidian")]
extern "C" {
    pub type Notice;

    #[wasm_bindgen(constructor)]
    pub fn new(message: JsString) -> Notice;
}

#[wasm_bindgen(module = "obsidian")]
extern "C" {
    pub type Editor;

    #[wasm_bindgen(structural, method)]
    pub fn replaceSelection(this: &Editor, replacement: JsString, origin: Option<JsString>);

    #[wasm_bindgen(structural, method)]
    pub fn getSelection(this: &Editor) -> JsString;
}

#[wasm_bindgen(module = "obsidian")]
extern "C" {
    pub type MarkdownView;

    #[wasm_bindgen(method, getter)]
    pub fn file(this: &MarkdownView) -> TFile;

    #[wasm_bindgen(method, getter)]
    pub fn app(this: &MarkdownView) -> App;

    #[wasm_bindgen(method, getter)]
    pub fn vault(this: &App) -> Vault;
    pub type Vault;

    pub type App;

    pub type TFile;

    #[wasm_bindgen(method)]
    pub fn read(this: &Vault, file: TFile) -> js_sys::Promise;
}
