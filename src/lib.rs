mod commands;
mod obsidian;
mod tasks;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn onload(plugin: &obsidian::Plugin) {
    macro_rules! add_command {
        ($command:ident) => {
            plugin.addCommand(JsValue::from(commands::$command::default()));
        };
    }
    add_command!(Echo);
    add_command!(ParseTask);
    add_command!(ParseFile);
    add_command!(Yoink);
}
