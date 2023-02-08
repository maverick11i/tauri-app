use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let greet_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg =
                        invoke("greet", to_value(&GreetArgs { name: &*name }).unwrap()).await;
                    log(&new_msg.as_string().unwrap());
                    greet_msg.set(new_msg.as_string().unwrap());
                });

                || {}
            },
            name2,
        );
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |_| {
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="container">
            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://yew.rs" target="_blank">
                    <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                </a>
            </div>

            <p>{"Click on the Tauri and Yew logos to learn more."}</p>

            <p>
                {"Recommended IDE setup: "}
                <a href="https://code.visualstudio.com/" target="_blank">{"VS Code"}</a>
                {" + "}
                <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">{"Tauri"}</a>
                {" + "}
                <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">{"rust-analyzer"}</a>
            </p>

            <div class="row">
                <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
                <button type="button" onclick={greet}>{"Greet"}</button>
            </div>

            <p><b>{ &*greet_msg }</b></p>
        </main>
    }
}

// use wasm_bindgen::JsCast;
// use web_sys::{EventTarget, HtmlInputElement, HtmlTextAreaElement};
// use yew::prelude::*;

// pub struct App {
//     data: String,
// }

// pub enum Msg {
//     InputTitle(String),
//     Hover,
// }

// impl Component for App {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {
//             data: String::new(),
//         }
//     }

//     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::InputTitle(arg) => {
//                 self.data = arg;
//                 true
//             }
//             Msg::Hover => {
//                 log::info!("{:?}", self.data.clone());
//                 true
//             }
//         }
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         let link = ctx.link();

//         let input = link.batch_callback(|e: InputEvent| {
//             let target = e
//                 .target()
//                 .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
//             target.map(|input| Msg::InputTitle(input.value()))
//         });
//         let textarea = link.batch_callback(|e: InputEvent| {
//             let target = e
//                 .target()
//                 .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
//             target.map(|input| Msg::InputTitle(input.value()))
//         });
//         let hover = link.batch_callback(|e: MouseEvent| {
//             let target: Option<EventTarget> = e.target();
//             let hover = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
//             hover.map(|_| Msg::Hover)
//         });
//         let hover_textarea = link.batch_callback(|e: MouseEvent| {
//             let target = e
//                 .target()
//                 .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
//             target.map(|_| Msg::Hover)
//         });

//         html! {
//             <>
//                 <input type="text" oninput={ input } onmouseover={ hover } />
//                 <textarea oninput={ textarea } onmouseover={ hover_textarea } />
//                 <p>{ self.data.clone() }</p>
//             </>
//         }
//     }
// }
