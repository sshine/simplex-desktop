use yew::prelude::*;


#[function_component(Contactlist)]
pub fn contact_list() -> Html {
    // dummy data
    let contacts = vec![("Josh", "Hey"), ("Bob", "How are you?"), ("Alice", "Connected")];
    let contacts: Vec<(String, String)> = contacts.iter().map(|(n, m)| (n.to_string(), m.to_string())).collect();

    html! {
        <>
            <div class="contacts">
                {{  
                    html! {
                        contacts.iter().map(|(n, m)| {
                            html! {
                                <div class="contact">
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
