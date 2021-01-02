use jinya_ui::layout::page::*;
use jinya_ui::widgets::alert::*;
use jinya_ui::layout::row::*;
use yew::prelude::*;

pub struct OneLineAlerts;

impl Component for OneLineAlerts {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        OneLineAlerts {}
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <Page>
                <Row>
                    <Alert message="One line primary alert" alert_type=AlertType::Primary/>
                </Row>
                <Row>
                    <Alert message="One line positive alert" alert_type=AlertType::Positive/>
                </Row>
                <Row>
                    <Alert message="One line negative alert" alert_type=AlertType::Negative/>
                </Row>
                <Row>
                    <Alert message="One line information alert" alert_type=AlertType::Information/>
                </Row>
            </Page>
        }
    }
}