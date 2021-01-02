use jinya_ui::layout::page::*;
use jinya_ui::widgets::form::multi_select::*;
use yew::prelude::*;
use yew::services::ConsoleService;
use jinya_ui::layout::form::*;

pub struct MultiSelects {
    link: ComponentLink<Self>,
    selected_items: Vec<MultiSelectItem>,
    options: Vec<MultiSelectItem>,
    original_options: Vec<MultiSelectItem>,
}

pub enum Msg {
    DefaultSelect(MultiSelectItem),
    DefaultDeselect(MultiSelectItem),
    DefaultFilter(String),
}

impl Component for MultiSelects {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MultiSelects {
            link,
            selected_items: vec![
                MultiSelectItem {
                    value: "option1".to_string(),
                    text: "Option 1".to_string(),
                },
                MultiSelectItem {
                    value: "option2".to_string(),
                    text: "Option 2".to_string(),
                },
            ],
            options: vec![
                MultiSelectItem {
                    value: "option3".to_string(),
                    text: "Option 3".to_string(),
                },
                MultiSelectItem {
                    value: "option4".to_string(),
                    text: "Option 4".to_string(),
                },
                MultiSelectItem {
                    value: "option5".to_string(),
                    text: "Option 5".to_string(),
                },
                MultiSelectItem {
                    value: "option6".to_string(),
                    text: "Option 6".to_string(),
                },
                MultiSelectItem {
                    value: "option7".to_string(),
                    text: "Option 7".to_string(),
                },
                MultiSelectItem {
                    value: "option8".to_string(),
                    text: "Option 8".to_string(),
                },
                MultiSelectItem {
                    value: "option9".to_string(),
                    text: "Option 9".to_string(),
                },
                MultiSelectItem {
                    value: "option10".to_string(),
                    text: "Option 10".to_string(),
                },
                MultiSelectItem {
                    value: "option11".to_string(),
                    text: "Option 11".to_string(),
                },
                MultiSelectItem {
                    value: "option12".to_string(),
                    text: "Option 12".to_string(),
                },
                MultiSelectItem {
                    value: "option13".to_string(),
                    text: "Option 13".to_string(),
                },
                MultiSelectItem {
                    value: "option14".to_string(),
                    text: "Option 14".to_string(),
                },
                MultiSelectItem {
                    value: "option15".to_string(),
                    text: "Option 15".to_string(),
                },
                MultiSelectItem {
                    value: "option16".to_string(),
                    text: "Option 16".to_string(),
                },
            ],
            original_options: vec![
                MultiSelectItem {
                    value: "option1".to_string(),
                    text: "Option 1".to_string(),
                },
                MultiSelectItem {
                    value: "option2".to_string(),
                    text: "Option 2".to_string(),
                },
                MultiSelectItem {
                    value: "option3".to_string(),
                    text: "Option 3".to_string(),
                },
                MultiSelectItem {
                    value: "option4".to_string(),
                    text: "Option 4".to_string(),
                },
                MultiSelectItem {
                    value: "option5".to_string(),
                    text: "Option 5".to_string(),
                },
                MultiSelectItem {
                    value: "option6".to_string(),
                    text: "Option 6".to_string(),
                },
                MultiSelectItem {
                    value: "option7".to_string(),
                    text: "Option 7".to_string(),
                },
                MultiSelectItem {
                    value: "option8".to_string(),
                    text: "Option 8".to_string(),
                },
                MultiSelectItem {
                    value: "option9".to_string(),
                    text: "Option 9".to_string(),
                },
                MultiSelectItem {
                    value: "option10".to_string(),
                    text: "Option 10".to_string(),
                },
                MultiSelectItem {
                    value: "option11".to_string(),
                    text: "Option 11".to_string(),
                },
                MultiSelectItem {
                    value: "option12".to_string(),
                    text: "Option 12".to_string(),
                },
                MultiSelectItem {
                    value: "option13".to_string(),
                    text: "Option 13".to_string(),
                },
                MultiSelectItem {
                    value: "option14".to_string(),
                    text: "Option 14".to_string(),
                },
                MultiSelectItem {
                    value: "option15".to_string(),
                    text: "Option 15".to_string(),
                },
                MultiSelectItem {
                    value: "option16".to_string(),
                    text: "Option 16".to_string(),
                },
            ],
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::DefaultDeselect(value) => {
                let index = self.selected_items.binary_search_by(|item| item.value.cmp(&value.value));
                if index.is_ok() {
                    self.selected_items.remove(index.unwrap());
                }
            }
            Msg::DefaultSelect(value) => {
                self.selected_items.push(value.clone());
                let options_idx = self.options.binary_search_by(|item| item.value.cmp(&value.value));
                if options_idx.is_ok() {
                    self.options.remove(options_idx.unwrap());
                }
            }
            Msg::DefaultFilter(value) => {
                let options = &self.original_options;
                let mut new_options = vec![]; // options.iter().filter(|item| item.value.to_lowercase().contains(value)).collect::<Vec<MultiSelectItem>>();
                for option in options {
                    if option.text.to_lowercase().contains(&value.to_lowercase()) {
                        ConsoleService::log(format!("{} {}", option.text, option.value).as_str());
                        new_options.push(option.to_owned())
                    }
                }
                ConsoleService::log(format!("{}", new_options.len()).as_str());
                self.options = new_options
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Page>
                <h1>{"MultiSelects"}</h1>
                <Form>
                    <MultiSelect
                        placeholder="Default placeholder"
                        label="Default multi select"
                        selected_items=&self.selected_items
                        options=&self.options
                        on_deselect=self.link.callback(|item| Msg::DefaultDeselect(item))
                        on_select=self.link.callback(|item| Msg::DefaultSelect(item))
                        on_filter=self.link.callback(|keyword| Msg::DefaultFilter(keyword))
                    />
                </Form>
            </Page>
        }
    }
}
