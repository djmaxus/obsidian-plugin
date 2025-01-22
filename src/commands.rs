use std::convert::identity;

use crate::obsidian::{self, TFile, Vault};
use js_sys::JsString;
use pulldown_cmark::{Event, Options, Parser};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

trait ErgoInto: Sized {
    fn ergo_into<I: From<Self>>(self) -> I {
        I::from(self)
    }
}
impl<F> ErgoInto for F {}

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

macro_rules! callback {
    ($struct_name:ident,$body:block) => {
        #[wasm_bindgen]
        impl $struct_name {
            #[wasm_bindgen(js_name = editorCallback)]
            pub async fn callback(&self) {
                $body
            }
        }
    };
}

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

command!(Info, "plugin info");
callback!(Info, {
    obsidian::Notice::new("dmaxus/obsidian-plugin".into());
});

command!(EchoSelection, "echo selection");
editor_callback!(EchoSelection, editor, _mv, {
    obsidian::Notice::new(editor.getSelection());
});

async fn file_to_js_string(vault: &Vault, file: TFile) -> JsString {
    vault
        .read(file)
        .ergo_into::<JsFuture>()
        .await
        .unwrap() // Promise<JsString> implied
        .ergo_into::<JsString>()
}

async fn string_to_file(string: String, vault: &Vault, file: TFile) {
    vault
        .modify(file, JsString::from(string), None)
        .ergo_into::<JsFuture>()
        .await
        .unwrap();
}

fn format_string(string: &str) -> String {
    format_string_with(string, identity)
}

fn format_string_with<'a, F, I>(string: &'a str, f: F) -> String
where
    F: Fn(Parser<'a>) -> I,
    I: IntoIterator<Item = Event<'a>>,
{
    let parser = pulldown_cmark::Parser::new_ext(string, Options::ENABLE_TASKLISTS);

    let modified = f(parser);

    let mut output = String::new();
    pulldown_cmark_to_cmark::cmark(modified.into_iter(), &mut output).unwrap();
    output
}

command!(Format, "format document");
editor_callback!(Format, _editor, mv, {
    let vault = mv.app().vault();
    let file = mv.file();

    let string = file_to_js_string(&vault, file).await.ergo_into::<String>();

    let formatted = format_string(&string);
    let formatted_again = format_string(&formatted);
    if formatted != formatted_again {
        obsidian::Notice::new("Formatting is not idempotent".into());
    }

    string_to_file(formatted_again, &vault, mv.file()).await;

    obsidian::Notice::new("Formatted".into());
});
