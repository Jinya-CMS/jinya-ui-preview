use jinya_ui::layout::button_row::*;
use jinya_ui::layout::page::*;
use jinya_ui::widgets::button::*;
use jinya_ui::widgets::floating_action_button::*;
use jinya_ui::widgets::toast::Toast;
use yew::prelude::*;

pub struct Toasts {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Primary,
    Positive,
    Negative,
    Information,
}

impl Component for Toasts {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Toasts {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Primary => Toast::primary_toast("Successfully saved the artist".to_string()),
            Msg::Positive => Toast::positive_toast("Successfully saved the artist".to_string()),
            Msg::Negative => Toast::negative_toast("Successfully saved the artist".to_string()),
            Msg::Information => Toast::information_toast("Successfully saved the artist".to_string()),
        }

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
                    <Button on_click=self.link.callback(|_| Msg::Primary) button_type=ButtonType::Primary label="Primary toast"></Button>
                    <Button on_click=self.link.callback(|_| Msg::Positive) button_type=ButtonType::Positive label="Positive toast"></Button>
                    <Button on_click=self.link.callback(|_| Msg::Negative) button_type=ButtonType::Negative label="Negative toast"></Button>
                    <Button on_click=self.link.callback(|_| Msg::Information) button_type=ButtonType::Information label="Information toast"></Button>
                </ButtonRow>
            </Page>
        }
    }
}
