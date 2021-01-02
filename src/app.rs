use jinya_ui::widgets::menu::bar::*;
use jinya_ui::widgets::menu::item::*;
use jinya_ui::widgets::toast::Toast;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew_router::{prelude::*, Switch};

use crate::pages::buttons::*;
use crate::pages::dialogs::confirmation::*;
use crate::pages::dialogs::content::*;
use crate::pages::form::checkbox::*;
use crate::pages::form::dropdown::*;
use crate::pages::form::file_upload::*;
use crate::pages::form::input::*;
use crate::pages::form::multi_select::*;
use crate::pages::form::radio::*;
use crate::pages::index::*;
use crate::pages::listing::cards::*;
use crate::pages::listing::table::*;
use crate::pages::multiline_alert::*;
use crate::pages::oneline_alert::*;
use crate::pages::toasts::*;
use crate::pages::tabs::*;
use crate::pages::splitview::*;

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "/buttons"]
    Buttons,
    #[to = "/inputs"]
    Inputs,
    #[to = "/checkboxes"]
    Checkboxes,
    #[to = "/radios"]
    Radios,
    #[to = "/dropdowns"]
    Dropdowns,
    #[to = "/multi-select"]
    MultiSelect,
    #[to = "/upload"]
    FileUploads,
    #[to = "/card"]
    Cards,
    #[to = "/confirmation"]
    Confirmation,
    #[to = "/content-dialog"]
    ContentDialog,
    #[to = "/alert-oneline"]
    OneLineAlert,
    #[to = "/alert-multiline"]
    MultiLineAlert,
    #[to = "/toasts"]
    Toasts,
    #[to = "/table"]
    Tables,
    #[to = "/tabs"]
    Tabs,
    #[to = "/splitviews"]
    SplitViews,
    #[to = "/"]
    Index,
}

pub enum Msg {
    Keyword(String),
    Search(String),
}

pub struct Model {
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Search(keyword) => Toast::information_toast(format!("Searched for {}", keyword)),
            Msg::Keyword(keyword) => ConsoleService::log(keyword.as_str()),
        }

        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let widget_groups = vec![
            SubItemGroup {
                title: "Buttons".to_string(),
                items: vec![
                    SubItem {
                        label: "Buttons".to_string(),
                        route: Some(&AppRoute::Buttons),
                        on_click: None,
                    },
                ],
            },
            SubItemGroup {
                title: "Dialogs".to_string(),
                items: vec![
                    SubItem {
                        label: "Confirmation".to_string(),
                        route: Some(&AppRoute::Confirmation),
                        on_click: None,
                    },
                    SubItem {
                        label: "Content".to_string(),
                        route: Some(&AppRoute::ContentDialog),
                        on_click: None,
                    },
                ],
            },
            SubItemGroup {
                title: "Tabs".to_string(),
                items: vec![
                    SubItem {
                        label: "Tabs".to_string(),
                        route: Some(&AppRoute::Tabs),
                        on_click: None,
                    },
                ],
            },
        ];

        let listing_groups = vec![
            SubItemGroup {
                title: "Cards".to_string(),
                items: vec![
                    SubItem {
                        label: "Cards".to_string(),
                        route: Some(&AppRoute::Cards),
                        on_click: None,
                    }
                ],
            },
            SubItemGroup {
                title: "Tables".to_string(),
                items: vec![
                    SubItem {
                        label: "Tables".to_string(),
                        route: Some(&AppRoute::Tables),
                        on_click: None,
                    }
                ],
            },
        ];

        let layout_groups = vec![
            SubItemGroup {
                title: "Views".to_string(),
                items: vec![
                    SubItem {
                        label: "Split view".to_string(),
                        route: Some(&AppRoute::SplitViews),
                        on_click: None,
                    }
                ],
            },
        ];

        let form_groups = vec![
            SubItemGroup {
                title: "Select".to_string(),
                items: vec![
                    SubItem {
                        label: "Checkboxes".to_string(),
                        route: Some(&AppRoute::Checkboxes),
                        on_click: None,
                    },
                    SubItem {
                        label: "Radios".to_string(),
                        route: Some(&AppRoute::Radios),
                        on_click: None,
                    },
                    SubItem {
                        label: "Dropdowns".to_string(),
                        route: Some(&AppRoute::Dropdowns),
                        on_click: None,
                    },
                    SubItem {
                        label: "Multi select".to_string(),
                        route: Some(&AppRoute::MultiSelect),
                        on_click: None,
                    },
                ],
            },
            SubItemGroup {
                title: "Input".to_string(),
                items: vec![
                    SubItem {
                        label: "Text fields".to_string(),
                        route: Some(&AppRoute::Inputs),
                        on_click: None,
                    },
                    SubItem {
                        label: "File uploads".to_string(),
                        route: Some(&AppRoute::FileUploads),
                        on_click: None,
                    },
                ],
            },
        ];

        let messages_groups = vec![
            SubItemGroup {
                title: "Alerts".to_string(),
                items: vec![
                    SubItem {
                        label: "One line alerts".to_string(),
                        route: Some(&AppRoute::OneLineAlert),
                        on_click: None,
                    },
                    SubItem {
                        label: "Multi line alerts".to_string(),
                        route: Some(&AppRoute::MultiLineAlert),
                        on_click: None,
                    },
                ],
            },
            SubItemGroup {
                title: "Notifications".to_string(),
                items: vec![
                    SubItem {
                        label: "Toasts".to_string(),
                        route: Some(&AppRoute::Toasts),
                        on_click: None,
                    },
                ],
            },
        ];

        html! {
            <main>
                <MenuBar title="Sample app" search_placeholder="Search" on_keyword=self.link.callback(|e| Msg::Keyword(e)) on_search=self.link.callback(|e| Msg::Search(e))>
                    <MenuItem<AppRoute> groups=widget_groups label="Widgets" />
                    <MenuItem<AppRoute> groups=form_groups label="Forms" />
                    <MenuItem<AppRoute> groups=messages_groups label="Messages" />
                    <MenuItem<AppRoute> groups=listing_groups label="Listing" />
                    <MenuItem<AppRoute> groups=layout_groups label="Layout" />
                </MenuBar>
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::Buttons => html! {<Buttons />},
                            AppRoute::Checkboxes => html! {<Checkboxes />},
                            AppRoute::Radios => html! {<Radios />},
                            AppRoute::Inputs => html! {<Inputs />},
                            AppRoute::Dropdowns => html! {<Dropdowns />},
                            AppRoute::MultiSelect => html! {<MultiSelects />},
                            AppRoute::FileUploads => html! {<FileUploads />},
                            AppRoute::Cards => html! {<Cards />},
                            AppRoute::Confirmation => html! {<Confirmation />},
                            AppRoute::ContentDialog => html! {<ContentDialogs />},
                            AppRoute::OneLineAlert => html! {<OneLineAlerts />},
                            AppRoute::MultiLineAlert => html! {<MultiLineAlerts />},
                            AppRoute::Toasts => html! {<Toasts />},
                            AppRoute::Tables => html! {<Tables />},
                            AppRoute::Tabs => html! {<Tabs />},
                            AppRoute::SplitViews => html! {<SplitViews />},
                            AppRoute::Index => html! {<Index />},
                        }
                    })
                />
            </main>
        }
    }
}