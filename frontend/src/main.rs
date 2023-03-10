use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

mod components;
use components::{chat::Chat, settings::Settings, sidebar::SideBar, welcome::Welcome};

fn main() {
    yew::Renderer::<App>::new().render();
}

// I wish there was an automatic way to do this
// But this enum represents all the components that can be in the content-view (aka left side of
// the screen)
#[derive(Debug, Default)]
pub enum ComponentOption {
    Chat,
    #[default]
    Welcome,
    Settings,
}

pub enum Msg {
    ChangeView(ComponentOption),
}

#[derive(Debug, Default)]
struct App {
    currect_page: ComponentOption,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeView(component) => self.currect_page = component,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="app-container">
                <SideBar
                    cb={ctx.link().callback(move |msg| msg)}
                />
                <div class="content-view">
                    {
                        match self.currect_page {
                            ComponentOption::Welcome => {
                                html! {
                                    <Welcome/>
                                }
                            }
                            ComponentOption::Chat => {
                                html! {
                                    <Chat/>
                                }
                            }
                            ComponentOption::Settings => {
                                html! {
                                    <Settings/>
                                }
                            }
                        }
                    }
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeHello, catch)]
    pub async fn hello(name: String) -> Result<JsValue, JsValue>;
}

fn _update_welcome_message(welcome: UseStateHandle<String>, name: String) {
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
