use reqwasm::http::Request;
use yew::prelude::*;

mod contact;
use contact::{Contact, ContactDetails, ContactsList};

#[function_component(App)]
fn app() -> Html {
    let selected_contact = use_state(|| None);

    let on_contact_select = {
        let selected_contact = selected_contact.clone();
        Callback::from(move |contact: Contact| selected_contact.set(Some(contact)))
    };

    let details = selected_contact.as_ref().map(|contact| {
        html! {
            <ContactDetails contact={contact.clone()} />
        }
    });

    let contacts = use_state(Vec::new);

    {
        let contacts = contacts.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_contacts: Vec<Contact> =
                        Request::get("http://localhost:8000/contacts")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    contacts.set(fetched_contacts);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <>
            <h1>{ "Contacts" }</h1>
            <ContactsList contacts={(*contacts).clone()} on_click={on_contact_select} />
            { for details }
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
