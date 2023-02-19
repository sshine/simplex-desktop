use yew::prelude::*;

#[function_component(NewConn)]
fn new_connection_button() -> Html {
    html! {
        <button class="big-btn btn">
            { "New chat" }
        </button>
    }
}

#[function_component(Welcome)]
pub fn welcome() -> Html {
    // here we could maybe have shortcuts to
    // - recent conversations
    // - conversations with unread messages
    // - favourites?
    // - making a new connection?
    // - and so on

    html! {
        <div class="welcome-container">
            <div class="mid">
                <h2 class="center-text">{ "SimpleX Desktop v0.1.0" }</h2>
                <p>{ "Start by making a connection" }</p>
            </div>
            <div class="floating-btn">
                <NewConn/>
            </div>
        </div>
    }
}
