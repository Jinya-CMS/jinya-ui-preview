use jinya_ui::layout::page::*;
use jinya_ui::layout::row::*;
use jinya_ui::widgets::form::input::*;
use jinya_ui::widgets::tab::*;
use yew::prelude::*;

pub struct Tabs {
    link: ComponentLink<Self>,
    default_value: String,
}

pub enum Msg {
    DefaultInput(String)
}

impl Component for Tabs {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Tabs {
            link,
            default_value: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::DefaultInput(value) => self.default_value = value
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <Page>
                <Row>
                    <TabControl pages=self.get_pages() />
                </Row>
            </Page>
        }
    }
}

impl Tabs {
    fn get_pages(&self) -> Vec<TabPage> {
        vec![
            TabPage::new(
                "Tab 1".to_string(),
                html! {
                        <>
                            <Input placeholder="Default placeholder 1" label="Default input 1" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                            <Input placeholder="Default placeholder 1" label="Default input 1" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                            <Input placeholder="Default placeholder 1" label="Default input 1" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                            <Input placeholder="Default placeholder 1" label="Default input 1" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                            <Input placeholder="Default placeholder 1" label="Default input 1" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                        </>
                    },
                "tab1".to_string(),
            ),
            TabPage::new(
                "Tab 2".to_string(),
                html! {
                        <>
                            <Input placeholder="Default placeholder 2" label="Default input 2" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                            <Input placeholder="Default placeholder 2" label="Default input 2" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                            <Input placeholder="Default placeholder 2" label="Default input 2" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                            <Input placeholder="Default placeholder 2" label="Default input 2" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                            <Input placeholder="Default placeholder 2" label="Default input 2" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                        </>
                    },
                "tab2".to_string(),
            ),
            TabPage::new(
                "Tab 3".to_string(),
                html! {
                        <>
                            <Input placeholder="Default placeholder 3" label="Default input 3" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                            <Input placeholder="Default placeholder 3" label="Default input 3" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                            <Input placeholder="Default placeholder 3" label="Default input 3" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                            <Input placeholder="Default placeholder 3" label="Default input 3" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                            <Input placeholder="Default placeholder 3" label="Default input 3" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                        </>
                    },
                "tab3".to_string(),
            ),
            TabPage::new(
                "Tab 4".to_string(),
                html! {
                        <>
                            <Input placeholder="Default placeholder 4" label="Default input 4" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                            <Input placeholder="Default placeholder 4" label="Default input 4" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                            <Input placeholder="Default placeholder 4" label="Default input 4" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                            <Input placeholder="Default placeholder 4" label="Default input 4" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                            <Input placeholder="Default placeholder 4" label="Default input 4" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.default_value />
                        </>
                    },
                "tab4".to_string(),
            ),
        ]
    }
}