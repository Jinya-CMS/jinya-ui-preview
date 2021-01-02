use jinya_ui::layout::button_row::*;
use jinya_ui::layout::page::*;
use jinya_ui::widgets::button::*;
use yew::prelude::*;
use yew_router::{prelude::*};

use crate::app::AppRoute;

pub struct Index {
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Index {
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
                <RouterButton<AppRoute> route=AppRoute::Buttons>
                    {"Buttons"}
                </RouterButton<AppRoute>>
            </Page>
        }
    }
}
