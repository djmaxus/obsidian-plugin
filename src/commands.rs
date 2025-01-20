use crate::obsidian;
use crate::tasks;
use js_sys::JsString;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

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
command!(ParseTask, "parse task from selection");
command!(ParseFile, "parse current file");

macro_rules! editor_callback {
    ($struct_name:ident,$editor:ident,$markdown_view:ident,$body:block) => {
        #[wasm_bindgen]
        impl $struct_name {
            #[wasm_bindgen(js_name = editorCallback)]
            pub async fn editor_callback(
                &self,
                $editor: obsidian::Editor,
                $markdown_view: obsidian::MarkdownView,
            ) {
                $body
            }
        }
    };
}

#[wasm_bindgen]
impl Echo {
    pub fn callback(&self) {
        obsidian::Notice::new("dmaxus/obsidian-plugin".into());
    }
}

editor_callback!(ParseTask, editor, _mv, {
    let selection = editor.getSelection();
    let selection: String = selection.into();
    if let Some(task) = tasks::Task::from_markdown(&selection) {
        let json = serde_json::to_string_pretty(&task).unwrap();
        obsidian::Notice::new(json.into());
    } else {
        obsidian::Notice::new("Not a task".into());
    }
});

trait ErgoInto: Sized {
    fn ergo_into<I: From<Self>>(self) -> I {
        I::from(self)
    }
}

impl<F> ErgoInto for F {}

editor_callback!(ParseFile, _editor, markdown_view, {
    let file = markdown_view.file();
    let vault = markdown_view.app().vault();

    let content = vault
        .read(file)
        .ergo_into::<JsFuture>()
        .await
        .unwrap()
        .ergo_into::<JsString>();

    obsidian::Notice::new(content);
});

command!(Yoink, "yoink!");

editor_callback!(Yoink, _e, mv, {
    let file = mv.file();
    let vault = mv.app().vault();

    let content = vault
        .read(file)
        .ergo_into::<JsFuture>()
        .await
        .unwrap()
        .ergo_into::<JsString>();

    obsidian::Notice::new(content);

    vault
        .modify(mv.file(), JsString::from(""), None)
        .ergo_into::<JsFuture>()
        .await
        .unwrap();

    obsidian::Notice::new(JsString::from("yoinked!"));
});
