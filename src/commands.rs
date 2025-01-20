use crate::obsidian;
use crate::tasks;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

macro_rules! command {
    ($struct_name:ident,$name:expr) => {
        #[wasm_bindgen]
        pub struct $struct_name {
            id: JsString,
            name: JsString,
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self {
                    id: stringify!($struct_name).into(),
                    name: $name.into(),
                }
            }
        }

        #[wasm_bindgen]
        impl $struct_name {
            #[wasm_bindgen(getter)]
            pub fn id(&self) -> JsString {
                self.id.clone()
            }

            #[wasm_bindgen(setter)]
            pub fn set_id(&mut self, id: &str) {
                self.id = id.into()
            }

            #[wasm_bindgen(getter)]
            pub fn name(&self) -> JsString {
                self.name.clone()
            }

            #[wasm_bindgen(setter)]
            pub fn set_name(&mut self, name: &str) {
                self.name = JsString::from(name)
            }
        }
    };
}

command!(Echo, "echo");

#[wasm_bindgen]
impl Echo {
    pub fn callback(&self) {
        obsidian::Notice::new("dmaxus/obsidian-plugin");
    }
}

command!(ParseTask, "parse task from selection");

#[wasm_bindgen]
impl ParseTask {
    #[wasm_bindgen(js_name = editorCallback)]
    pub fn editor_callback(&self, editor: obsidian::Editor, _: obsidian::MarkdownView) {
        let selection = editor.getSelection();
        let selection: String = selection.into();
        if let Some(task) = tasks::Task::from_markdown(&selection) {
            let json = serde_json::to_string_pretty(&task).unwrap();
            obsidian::Notice::new(&json);
        } else {
            obsidian::Notice::new("Not a task");
        }
    }
}
