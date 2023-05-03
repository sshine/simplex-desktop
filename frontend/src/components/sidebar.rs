use super::contacts::*;
use crate::ComponentOption::*;
use crate::Msg;
use yew::prelude::*;
// to be moved?
#[function_component(Search)]
pub fn search_box() -> Html {
    // TODO change into a <textarea>?
    html! {
        <div class="search-box">
            <input type="text" placeholder="Search"/>
        </div>
    }
}

#[function_component(Profile)]
pub fn current_profile(props: &Props) -> Html {
    // Todo: add profile picture
    let name = "User1".to_string();
    let cb = props.cb.clone();
    // We can make a macro for this to reduce repetitiveness
    let show_welcome = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        cb.emit(Msg::ChangeView(Welcome))
    });
    let cb = props.cb.clone();
    let show_settings = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        cb.emit(Msg::ChangeView(Settings))
    });
    html! {
        <div class="profile">
            <p class="name" onclick={show_welcome}>{name}</p>
            <button class="btn" onclick={show_settings}>{"settings"}</button>
        </div>
    }
}

#[function_component(SideBar)]
pub fn sidebar(props: &Props) -> Html {
    html! {
        <div class="sidebar">
            <div class="sticky-container">
                <Profile
                    cb={props.cb.clone()}
                />
                <Search/>
            </div>
            <Contactlist
                cb={props.cb.clone()}
            />
        </div>
    }
}
