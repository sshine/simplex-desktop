use yew::prelude::*;

#[function_component(Chat)]
pub fn chat() -> Html {
    // TODO add a toggle to contact info
    // TODO hide contact info if screen is too small for it
    // TODO some of these elements should be <button>s
    let messages = vec![
        "Connected",
        "Hello, how are you?",
        "Are you new on simplex?",
    ];
    html! {
        <div class="chat-container">
            <section class="chat-title">
                <p class="contact-name">
                {"@ Contact name "}
                </p>
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
            <div class="contact-info">
                <p>
                {"Name / Full Name"}
                </p>
                <p>
                {"Verify security code"}
                </p>
                <p>
                {"Contact preferences"}
                </p>
                <p class="text-accent">
                {"Switch receiving address"}
                </p>
                <p class="">
                { "Network status" }
                </p>
                <p>
                { "Receiving via" }
                </p>
                <p>
                {"Sending via"}
                </p>

                <p class="warning">
                { "Clear chat" }
                </p>
                <p class="warning-red">
                { "Delete contact" }
                </p>
                <p>
                { "Local name" }
                </p>
                <p>
                { "Database ID" }
                </p>
            </div>
            <div class="message-box">
                <textarea placeholder="Message @Bob">
                </textarea>
            </div>
        </div>
    }
}
