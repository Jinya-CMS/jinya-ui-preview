use jinya_ui::layout::button_row::*;
use jinya_ui::layout::page::*;
use jinya_ui::listing::card::*;
use jinya_ui::widgets::button::*;
use jinya_ui::widgets::dialog::confirmation::*;
use yew::prelude::*;
use yew::services::ConsoleService;

pub struct Confirmation {
    link: ComponentLink<Self>,
    negative_dialog_open: bool,
    primary_dialog_open: bool,
}

pub enum Msg {
    OpenNegativeConfirmation,
    NegativeApprove,
    NegativeDecline,
    OpenPrimaryConfirmation,
    PrimaryApprove,
    PrimaryDecline,
}

impl Component for Confirmation {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Confirmation {
            link,
            negative_dialog_open: false,
            primary_dialog_open: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::OpenNegativeConfirmation => self.negative_dialog_open = true,
            Msg::NegativeApprove => {
                ConsoleService::log("Negative approved");
                self.negative_dialog_open = false
            }
            Msg::NegativeDecline => {
                ConsoleService::log("Negative declined");
                self.negative_dialog_open = false
            }
            Msg::OpenPrimaryConfirmation => self.primary_dialog_open = true,
            Msg::PrimaryApprove => {
                ConsoleService::log("Primary approved");
                self.primary_dialog_open = false
            }
            Msg::PrimaryDecline => {
                ConsoleService::log("Primary declined");
                self.primary_dialog_open = false
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
                <h1>{"Cards"}</h1>
                <ButtonRow alignment=ButtonRowAlignment::Start>
                    <Button label="Open Primary" button_type=ButtonType::Primary on_click=self.link.callback(|_| Msg::OpenPrimaryConfirmation) />
                    <Button label="Open Negative" button_type=ButtonType::Negative on_click=self.link.callback(|_| Msg::OpenNegativeConfirmation) />
                </ButtonRow>
                <ConfirmationDialog
                    title="Primary confirmation dialog"
                    message="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                    decline_label="Decline"
                    approve_label="Confirm"
                    on_approve=self.link.callback(|_| Msg::PrimaryApprove)
                    on_decline=self.link.callback(|_| Msg::PrimaryDecline)
                    is_open=&self.primary_dialog_open
                />
                <ConfirmationDialog
                    title="Negative confirmation dialog"
                    dialog_type=DialogType::Negative
                    message="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                    decline_label="Decline"
                    approve_label="Confirm"
                    on_approve=self.link.callback(|_| Msg::NegativeApprove)
                    on_decline=self.link.callback(|_| Msg::NegativeDecline)
                    is_open=&self.negative_dialog_open
                />
            </Page>
        }
    }
}