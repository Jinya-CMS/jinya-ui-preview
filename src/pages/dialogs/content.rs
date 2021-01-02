use jinya_ui::widgets::form::input::*;
use jinya_ui::layout::button_row::*;
use jinya_ui::layout::page::*;
use jinya_ui::widgets::button::*;
use jinya_ui::widgets::dialog::content::*;
use yew::prelude::*;
use yew::services::ConsoleService;

pub struct ContentDialogs {
    link: ComponentLink<Self>,
    dialog_open: bool,
    value: String,
}

pub enum Msg {
    Open,
    Primary,
    Secondary,
    DefaultInput(String),
}

impl Component for ContentDialogs {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ContentDialogs {
            link,
            dialog_open: false,
            value: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Open => self.dialog_open = true,
            Msg::Primary => {
                ConsoleService::log("Negative approved");
                self.dialog_open = false
            }
            Msg::Secondary => {
                ConsoleService::log("Negative declined");
                self.dialog_open = false
            }
            Msg::DefaultInput(value) => self.value = value
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Page>
                <h1>{"Cards"}</h1>
                <ButtonRow alignment=ButtonRowAlignment::Start>
                    <Button label="Open Primary" button_type=ButtonType::Primary on_click=self.link.callback(|_| Msg::Open) />
                </ButtonRow>
                <ContentDialog
                    title="Content dialog"
                    secondary_label="Discard changes"
                    primary_label="Save changes"
                    on_primary=self.link.callback(|_| Msg::Primary)
                    on_secondary=self.link.callback(|_| Msg::Secondary)
                    is_open=&self.dialog_open
                >
                    <div>
                        <Input placeholder="Default placeholder" label="Default input" on_input=self.link.callback(|value| Msg::DefaultInput(value)) value=&self.value />
                    </div>
                </ContentDialog>
            </Page>
        }
    }
}