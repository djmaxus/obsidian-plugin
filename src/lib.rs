mod gtd;
mod obsidian;

use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn onload(plugin: &obsidian::Plugin) {
    let cmd = ExampleCommand {
        id: JsString::from("gtd-task-read"),
        name: JsString::from("Read selected task"),
    };
    plugin.addCommand(JsValue::from(cmd))
}

#[wasm_bindgen]
pub struct ExampleCommand {
    id: JsString,
    name: JsString,
}

#[wasm_bindgen]
impl ExampleCommand {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> JsString {
        self.id.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, id: &str) {
        self.id = JsString::from(id)
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> JsString {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: &str) {
        self.name = JsString::from(name)
    }

    // pub fn callback(&self) {
    //     obsidian::Notice::new("hello from rust");
    // }

    pub fn editorCallback(&self, editor: obsidian::Editor, view: obsidian::MarkdownView) {
        let selection = editor.getSelection();
        let selection: String = selection.into();
        if let Some(task) = gtd::Task::from_markdown(&selection) {
            let json = serde_json::to_string_pretty(&task).unwrap();
            obsidian::Notice::new(&json);
        } else {
            obsidian::Notice::new("Not a task");
        }
    }
}
