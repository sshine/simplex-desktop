use yew::prelude::*;

#[function_component(Chat)]
pub fn chat() -> Html {
    html! {
        <>
            <div class="chat">
                <p> {"HI!!"} </p>
            </div>
            <div class="message-box">
                <input type="text" placeholder="Message @Bob"/>
            </div>
        </>
    }
}
