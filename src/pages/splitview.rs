use jinya_ui::layout::page::*;
use jinya_ui::layout::split_view::*;
use jinya_ui::widgets::button::*;
use jinya_ui::widgets::floating_action_button::*;
use yew::prelude::*;

pub struct SplitViews {
    link: ComponentLink<Self>,
}

pub enum Msg {
    NoParam
}

impl Component for SplitViews {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        SplitViews {
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
                <SplitView left=self.get_left() right=self.get_right()>
                </SplitView>
            </Page>
        }
    }
}

impl SplitViews {
    fn get_left(&self) -> Html {
        html! {
            <>
                <Button on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Primary label="Primary button"></Button>
                <Button on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Primary label="Secondary button"></Button>
                <Button on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Primary label="Positive button"></Button>
                <Button on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Primary label="Negative button"></Button>
                <Button on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Primary label="Information button"></Button>
            </>
        }
    }
    fn get_right(&self) -> Html {
        html! {
            <>
                <Button on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Secondary label="Primary button"></Button>
                <Button on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Secondary label="Secondary button"></Button>
                <Button on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Secondary label="Positive button"></Button>
                <Button on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Secondary label="Negative button"></Button>
                <Button on_click=self.link.callback(|_| Msg::NoParam) button_type=ButtonType::Secondary label="Information button"></Button>
            </>
        }
    }
}