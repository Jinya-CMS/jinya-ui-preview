use jinya_ui::layout::form::*;
use jinya_ui::layout::page::*;
use jinya_ui::widgets::form::checkbox::*;
use yew::prelude::*;

pub struct Checkboxes {
    link: ComponentLink<Self>,
    default_checked: bool,
    positive_checked: bool,
    negative_checked: bool,
    negative_validation_message: String,
}

pub enum Msg {
    DefaultCheckbox,
    PositiveCheckbox,
    NegativeCheckbox,
}

impl Component for Checkboxes {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Checkboxes {
            link,
            default_checked: false,
            positive_checked: false,
            negative_checked: false,
            negative_validation_message: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::DefaultCheckbox => self.default_checked = !self.default_checked,
            Msg::PositiveCheckbox => self.positive_checked = !self.positive_checked,
            Msg::NegativeCheckbox => {
                self.negative_checked = !self.negative_checked;
                self.negative_validation_message = if self.negative_checked {
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
                <h1>{"Checkboxes"}</h1>
                <Form>
                    <Checkbox
                        label="Default checkbox"
                        on_change=self.link.callback(|_| Msg::DefaultCheckbox)
                        checked=&self.default_checked
                    />
                    <Checkbox
                        label="Disabled checkbox"
                        disabled=true
                        on_change=self.link.callback(|_| Msg::DefaultCheckbox)
                        checked=&self.default_checked
                    />
                    <Checkbox
                        label="Positive checkbox"
                        on_change=self.link.callback(|_| Msg::PositiveCheckbox)
                        state=CheckboxState::Positive
                        checked=&self.positive_checked
                    />
                    <Checkbox
                        label="Negative checkbox"
                        on_change=self.link.callback(|_| Msg::NegativeCheckbox)
                        state=CheckboxState::Negative
                        checked=&self.negative_checked
                        validation_message=&self.negative_validation_message
                    />
                </Form>
            </Page>
        }
    }
}
