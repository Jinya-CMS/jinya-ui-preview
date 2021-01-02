use jinya_ui::layout::button_row::*;
use jinya_ui::layout::page::*;
use jinya_ui::widgets::button::*;
use jinya_ui::widgets::floating_action_button::*;
use yew::prelude::*;

pub struct Buttons {
    link: ComponentLink<Self>,
}

pub enum Msg {
    NoParam
}

impl Component for Buttons {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Buttons {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Page>
                <h1>{"Buttons"}</h1>
                <ButtonRow alignment=ButtonRowAlignment::Start>
                    <Button on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Primary label="Primary button"></Button>
                    <Button on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Secondary label="Secondary button"></Button>
                    <Button on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Positive label="Positive button"></Button>
                    <Button on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Negative label="Negative button"></Button>
                    <Button on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Information label="Information button"></Button>
                </ButtonRow>
                <ButtonRow>
                    <Button small=true on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Primary label="Primary button"></Button>
                    <Button small=true on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Secondary label="Secondary button"></Button>
                    <Button small=true on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Positive label="Positive button"></Button>
                    <Button small=true on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Negative label="Negative button"></Button>
                    <Button small=true on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Information label="Information button"></Button>
                </ButtonRow>
                <Fab icon="file-upload" on_click=self.link.callback(|_| Msg::NoParam) />
            </Page>
        }
    }
}
