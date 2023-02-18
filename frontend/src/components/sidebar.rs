use super::contacts::*;
use yew::prelude::*;

// to be moved?
#[function_component(Search)]
pub fn search_box() -> Html {
    html! {
        <div class="search-box">
            <input type="text" placeholder="Search"/>
        </div>
    }
}

#[function_component(Profile)]
pub fn currecnt_profile() -> Html {
    // Todo: add profile picture
    let name = "User1".to_string();
    html! {
        <div class="profile">
            <p class="name">{name}</p>
            <button class="btn">{"settings"}</button>
        </div>
    }
}

#[function_component(SideBar)]
pub fn sidebar() -> Html {
    html! {
        <div class="sidebar">
            <div class="sticky-container">
                <Profile/>
                <Search/>
            </div>
            <Contactlist/>
        </div>
    }
}
