use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

enum Msg {
    AddOne,
}

#[derive(Debug, Default)]
struct App {
    value: u64,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>
                    { "You clicked " }
                    <button onclick={ctx.link().callback(|_| Msg::AddOne)}>{ "+1" }</button>
                    { " " }
                    { self.value }
                    { " times." }
                </p>
            </div>
        }
    }
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;
}

fn update_welcome_message(welcome: UseStateHandle<String>, name: String) {
    spawn_local(async move {
        // This will call our glue code all the way through to the tauri
        // back-end command and return the `Result<String, String>` as
        // `Result<JsValue, JsValue>`.
        match hello(name).await {
            Ok(message) => {
                welcome.set(message.as_string().unwrap());
            }
            Err(e) => {
                let window = window().unwrap();
                window.alert_with_message(&format!("Error: {e:?}")).unwrap();
            }
        }
    })
}
