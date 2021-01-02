use jinya_ui::layout::page::*;
use jinya_ui::listing::card::*;
use jinya_ui::widgets::floating_action_button::*;
use yew::prelude::*;

pub struct Cards {
    link: ComponentLink<Self>,
}

pub enum Msg {
    NoParam
}

impl Component for Cards {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Cards {
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Page>
                <h1>{"Cards"}</h1>
                <CardContainer>
                    <>
                        <Card title="Some card" src="https://source.unsplash.com/random">
                            <CardButton icon="plus" tooltip="Some card button" on_click=self.link.callback(|_| Msg::NoParam) />
                        </Card>
                        <Card title="Some card" src="https://source.unsplash.com/random">
                            <CardButton icon="plus" tooltip="Some card button" on_click=self.link.callback(|_| Msg::NoParam) />
                        </Card>
                        <Card title="Some card" src="https://source.unsplash.com/random">
                            <CardButton icon="plus" tooltip="Some card button" on_click=self.link.callback(|_| Msg::NoParam) />
                        </Card>
                        <Card title="Some card" src="https://source.unsplash.com/random">
                            <CardButton icon="plus" tooltip="Some card button" on_click=self.link.callback(|_| Msg::NoParam) />
                        </Card>
                        <Card title="Some card" src="https://source.unsplash.com/random">
                            <CardButton icon="plus" tooltip="Some card button" on_click=self.link.callback(|_| Msg::NoParam) />
                        </Card>
                        <Card title="Some card" src="https://source.unsplash.com/random">
                            <CardButton icon="plus" tooltip="Some card button" on_click=self.link.callback(|_| Msg::NoParam) />
                        </Card>
                        <Card title="Some card" src="https://source.unsplash.com/random">
                            <CardButton icon="plus" tooltip="Some card button" on_click=self.link.callback(|_| Msg::NoParam) />
                        </Card>
                        <Card title="Some card" src="https://source.unsplash.com/random">
                            <CardButton icon="plus" tooltip="Some card button" on_click=self.link.callback(|_| Msg::NoParam) />
                        </Card>
                        <Card title="Some card" src="https://source.unsplash.com/random">
                            <CardButton icon="plus" tooltip="Some card button" on_click=self.link.callback(|_| Msg::NoParam) />
                        </Card>
                        <Card title="Some card" src="https://source.unsplash.com/random">
                            <CardButton icon="plus" tooltip="Some card button" on_click=self.link.callback(|_| Msg::NoParam) />
                        </Card>
                        <Card title="Some card" src="https://source.unsplash.com/random">
                            <CardButton icon="plus" tooltip="Some card button" on_click=self.link.callback(|_| Msg::NoParam) />
                        </Card>
                    </>
                </CardContainer>
            </Page>
        }
    }
}