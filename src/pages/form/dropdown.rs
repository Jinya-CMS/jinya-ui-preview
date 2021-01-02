use jinya_ui::layout::page::*;
use jinya_ui::widgets::form::dropdown::*;
use jinya_ui::layout::form::*;
use yew::prelude::*;
use yew::services::ConsoleService;

pub struct Dropdowns {
    link: ComponentLink<Self>,
    default_value: String,
    positive_value: String,
    negative_value: String,
    negative_validation_message: String,
}

pub enum Msg {
    DefaultDropdown(String),
    PositiveDropdown(String),
    NegativeDropdown(String),
}

impl Component for Dropdowns {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Dropdowns {
            link,
            default_value: "".to_string(),
            positive_value: "".to_string(),
            negative_value: "".to_string(),
            negative_validation_message: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::DefaultDropdown(value) => self.default_value = value,
            Msg::PositiveDropdown(value) => self.positive_value = value,
            Msg::NegativeDropdown(value) => {
                ConsoleService::log(value.as_str());
                self.negative_value = value.to_string();
                self.negative_validation_message = if value.to_string().len() > 5 {
                    "valid".to_string()
                } else {
                    "invalid".to_string()
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let select_items = vec![
            DropdownItem {
                value: "option1".to_string(),
                text: "Option 1".to_string()
            }
        ];
        html! {
            <Page>
                <h1>{"Drodpdowns"}</h1>
                <Form>
                    <Dropdown
                        placeholder="Default placeholder"
                        label="Default dropdown"
                        on_select=self.link.callback(|value| Msg::DefaultDropdown(value))
                        items=&select_items
                    />
                    <Dropdown
                        placeholder="Disabled placeholder"
                        label="Disabled dropdown"
                        disabled=true
                        on_select=self.link.callback(|value| Msg::DefaultDropdown(value))
                        items=&select_items
                    />
                    <Dropdown
                        placeholder="Positive placeholder"
                        state=DropdownState::Positive
                        label="Positive dropdown"
                        on_select=self.link.callback(|value| Msg::PositiveDropdown(value))
                        items=&select_items
                    />
                    <Dropdown
                        placeholder="Negative placeholder"
                        validation_message=&self.negative_validation_message
                        state=DropdownState::Negative
                        label="Negative dropdown"
                        on_select=self.link.callback(move |value| Msg::NegativeDropdown(value))
                        items=&select_items
                    />
                </Form>
            </Page>
        }
    }
}
