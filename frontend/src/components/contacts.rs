use yew::prelude::*;

use crate::ComponentOption::*;
use crate::Msg;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub cb: Callback<Msg>,
}

#[function_component(Contactlist)]
pub fn contact_list(props: &Props) -> Html {
    // dummy data
    let contacts = vec![
        ("Josh", "Hey"),
        ("Bob", "How are you?"),
        ("Alice", "Connected"),
    ];
    // let contacts: Vec<(String, String)> = contacts
    //     .iter()
    //     .map(|(n, m)| (n.to_string(), m.to_string()))
    //     .collect();

    let cb = props.cb.clone();
    let onclick = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        cb.emit(Msg::ChangeView(Chat))
    });

    html! {
        <>
            <div class="contacts">
                {{
                    html! {
                        contacts.iter().map(|(n, m)| {
                            html! {
                                <div class="contact" onclick={onclick.clone()}>
                                    <p class="name"> { n } </p>
                                    <p class="msg-preview"> { m } </p>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                }}
            </div>
        </>
    }
}
