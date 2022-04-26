use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Contact {
    pub id: u32,
    pub name: String,
}

#[derive(Clone, Properties, PartialEq)]
pub struct ContactListProps {
    pub contacts: Vec<Contact>,
    pub on_click: Callback<Contact>,
}

#[function_component(ContactsList)]
pub fn contacts_list(ContactListProps { contacts, on_click }: &ContactListProps) -> Html {
    contacts
        .iter()
        .map(|contact| {
            let on_contact_select = {
                let on_click = on_click.clone();
                let contact = contact.clone();
                Callback::from(move |_| on_click.emit(contact.clone()))
            };

            html! {
                <p onclick={on_contact_select}>{contact.name.to_string()}</p>
            }
        })
        .collect()
}

#[derive(Clone, Properties, PartialEq)]
pub struct ContactDetailsProps {
    pub contact: Contact,
}

#[function_component(ContactDetails)]
pub fn contact_details(ContactDetailsProps { contact }: &ContactDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ format!("ID: {}",contact.id) }</h3>
            <h3>{ format!("Name: {}",contact.name) }</h3>
        </div>
    }
}
