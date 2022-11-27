use yew::prelude::*;

use crate::components::{CardGroup, EntryCard};
use crate::routes::entries::EntryInfo;

#[derive(Debug, Properties, PartialEq, Eq)]
pub struct Props {
    pub entries: Option<Vec<EntryInfo>>,
}

#[function_component(EntryList)]
pub fn entry_list(props: &Props) -> Html {
    let Props { entries } = props.clone();

    {
        if entries.is_none() {
            html! {
                {"Einträge werden geladen..."}
            }
        } else {
            entries
                .as_ref()
                .unwrap()
                .chunks(4)
                .map(|chunk| {
                    html! {
                        <CardGroup>
                        {
                            chunk.iter().map(|entry| {
                                html! {
                                    <EntryCard entry={entry.clone()} />
                                }
                            }).collect::<Html>()
                        }
                        </CardGroup>
                    }
                })
                .collect::<Html>()
        }
    }
}
