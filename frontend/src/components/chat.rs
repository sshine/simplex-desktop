use std::ops::Not;

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
    let sidebar_visible = use_state_eq(|| true);
    let toggle_sidebar = {
        let val = sidebar_visible.clone();
        Callback::from(move |_| {
            val.set(val.not());
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
                    messages.iter().map(|msg| {
                        html!{
                            <p class="message">
                                {msg}
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
                            <p>
                            {"Verify security code"}
                            </p>
                            <p>
                            {"Contact preferences"}
                            </p>
                            <Divider/>
                            <p class="text-accent">
                            {"Switch receiving address"}
                            </p>
                            <p>
                            { "Network status" }
                            </p>
                            <p>
                            { "Receiving via" }
                            </p>
                            <p>
                            {"Sending via"}
                            </p>
                            <Divider/>
                            <p class="warning">
                            { "Clear chat" }
                            </p>
                            <p class="warning-red">
                            { "Delete contact" }
                            </p>
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
