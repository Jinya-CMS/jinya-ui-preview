use jinya_ui::layout::page::*;
use jinya_ui::widgets::form::input::*;
use yew::prelude::*;
use jinya_ui::layout::form::*;

pub struct Inputs {
    link: ComponentLink<Self>,
    default_value: String,
    positive_value: String,
    negative_value: String,
    negative_validation_message: String,
}

pub enum Msg {
    DefaultInput(String),
    PositiveInput(String),
    NegativeInput(String),
}

impl Component for Inputs {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Inputs {
            link,
            default_value: "".to_string(),
            positive_value: "".to_string(),
            negative_value: "".to_string(),
            negative_validation_message: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::DefaultInput(value) => self.default_value = value,
            Msg::PositiveInput(value) => self.positive_value = value,
            Msg::NegativeInput(value) => {
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
        html! {
            <Page>
                <h1>{"Text fields"}</h1>
                <Form>
                    <Input placeholder="Default placeholder" label="Default input" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                    <Input placeholder="Disabled placeholder" disabled=true label="Disabled input" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                    <Input placeholder="Positive placeholder" state=InputState::Positive label="Positive input" on_input=self.link.callback(|value| Msg::PositiveInput(value)) value=&self.positive_value />
                    <Input placeholder="Negative placeholder" validation_message=&self.negative_validation_message state=InputState::Negative label="Negative input" on_input=self.link.callback(move |value| Msg::NegativeInput(value)) value=&self.negative_value />
                </Form>
            </Page>
        }
    }
}
