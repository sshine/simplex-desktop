use super::util::*;
use yew::prelude::*;

#[function_component(Chat)]
pub fn chat() -> Html {
    // TODO add a toggle to contact info
    // TODO hide contact info if screen is too small for it
    // TODO some of these elements should be <button>s
    // TODO add a profile picture
    let messages = vec![
        "Connected",
        "Hello, how are you?",
        "Are you new on simplex?",
    ];
    let messages: Vec<&str> = std::iter::repeat(messages).take(14).flatten().collect();

    let sidebar_visible = use_state_eq(|| true);
    let toggle_sidebar = {
        let val = sidebar_visible.clone();
        Callback::from(move |_| {
            val.set(!*val);
        })
    };
    html! {
        <div class={classes!("chat-container", sidebar_visible.then(|| "with-contact"))}>
            <section class="chat-title">
                <p class="contact-name">
                {"@ Contact name "}
                </p>
                <button class="btn dark" onclick={toggle_sidebar}>
                    {"Toggle sidebar"}
                </button>
            </section>
            <div class="chat">
                {
                    messages.iter().rev().enumerate().map(|(i, msg)| {
                        html!{
                            <p class="message">
                                {i}{msg}
                            </p>
                        }
                    }).collect::<Html>()
                }
            </div>
            {
                if *sidebar_visible {
                    html! {
                        <div class="contact-info">
                            <p>
                            {"Name / Full Name"}
                            </p>
                            <Divider/>
                            <button>
                            {"Verify security code"}
                            </button>
                            <button>
                            {"Contact preferences"}
                            </button>
                            <Divider/>
                            <button class="text-accent">
                            {"Switch receiving address"}
                            </button>
                            <button>
                            { "Network status" }
                            </button>
                            <button>
                            { "Receiving via" }
                            </button>
                            <button>
                            {"Sending via"}
                            </button>
                            <Divider/>
                            <button class="warning">
                            { "Clear chat" }
                            </button>
                            <button class="warning-red">
                            { "Delete contact" }
                            </button>
                            <Divider/>
                            <p>
                            { "Local name" }
                            </p>
                            <p>
                            { "Database ID" }
                            </p>
                        </div>
                    }
                } else {
                    html!{}
                }
            }
            <div class="message-box">
                <textarea placeholder="Message @Bob">
                </textarea>
            </div>
        </div>
    }
}
