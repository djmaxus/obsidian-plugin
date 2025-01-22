mod commands;
mod obsidian;
mod todo_list;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn onload(plugin: &obsidian::Plugin) {
    macro_rules! add_commands {
        () => {};
        ($command:ident) => {
            plugin.addCommand(JsValue::from(commands::$command::default()));
        };
        ($command:ident, $( $other_command:ident ),*) => {
            add_commands! {$command}
            add_commands! {$( $other_command ),*}
        };
    }

    add_commands![Info, Format, EchoSelection];
}
