use jinya_ui::layout::page::*;
use jinya_ui::widgets::form::radio::*;
use yew::prelude::*;
use jinya_ui::layout::form::*;

pub struct Radios {
    link: ComponentLink<Self>,
    default_checked: bool,
    positive_checked: bool,
    negative_checked: bool,
    negative_validation_message: String,
}

pub enum Msg {
    DefaultRadio,
    PositiveRadio,
    NegativeRadio,
}

impl Component for Radios {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Radios {
            link,
            default_checked: false,
            positive_checked: false,
            negative_checked: false,
            negative_validation_message: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::DefaultRadio => {
                self.negative_checked = false;
                self.positive_checked = false;
                self.default_checked = true;
            },
            Msg::PositiveRadio => {
                self.negative_checked = false;
                self.positive_checked = true;
                self.default_checked = false;
            },
            Msg::NegativeRadio => {
                self.negative_checked = true;
                self.positive_checked = false;
                self.default_checked = false;
            }
        }
        self.negative_validation_message = if self.negative_checked {
            "valid".to_string()
        } else {
            "invalid".to_string()
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Page>
                <h1>{"Radios"}</h1>
                <Form>
                    <Radio
                        label="Default Radio"
                        group="radios"
                        on_change=self.link.callback(|_| Msg::DefaultRadio)
                        checked=&self.default_checked
                        value="Radio 1"
                    />
                    <Radio
                        label="Disabled Radio"
                        disabled=true
                        group="radios"
                        on_change=self.link.callback(|_| Msg::DefaultRadio)
                        checked=&self.default_checked
                        value="Radio 1"
                    />
                    <Radio
                        label="Positive Radio"
                        group="radios"
                        on_change=self.link.callback(|_| Msg::PositiveRadio)
                        state=RadioState::Positive
                        checked=&self.positive_checked
                        value="Radio 2"
                    />
                    <Radio
                        label="Negative Radio"
                        group="radios"
                        on_change=self.link.callback(|_| Msg::NegativeRadio)
                        state=RadioState::Negative
                        checked=&self.negative_checked
                        validation_message=&self.negative_validation_message
                        value="Radio 3"
                    />
                </Form>
            </Page>
        }
    }
}
