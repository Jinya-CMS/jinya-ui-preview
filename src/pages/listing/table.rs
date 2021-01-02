use jinya_ui::layout::page::*;
use jinya_ui::listing::table::*;
use jinya_ui::widgets::floating_action_button::*;
use yew::prelude::*;
use yew::services::ConsoleService;

pub struct Tables {
    link: ComponentLink<Self>,
    selected: Option<usize>,
}

pub enum Msg {
    Select(usize),
}

impl Component for Tables {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Tables {
            link,
            selected: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Select(idx) => {
                self.selected = Option::from(idx);
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let headers = vec![
            TableHeader {
                title: "Header 1".to_string(),
                key: "column1".to_string(),
            },
            TableHeader {
                title: "Header 2".to_string(),
                key: "column2".to_string(),
            },
            TableHeader {
                title: "Header 3".to_string(),
                key: "column3".to_string(),
            },
            TableHeader {
                title: "Header 4".to_string(),
                key: "column4".to_string(),
            },
            TableHeader {
                title: "Header 5".to_string(),
                key: "column5".to_string(),
            },
        ];

        let range = 0..20;
        let rows = range.into_iter().enumerate().map(|(_, i)| {
            TableRow::new(vec![
                TableCell {
                    key: "column1".to_string(),
                    value: format!("Column 1 – Row {:?}", i),
                },
                TableCell {
                    key: "column2".to_string(),
                    value: format!("Column 2 – Row {:?}", i),
                },
                TableCell {
                    key: "column3".to_string(),
                    value: format!("Column 3 – Row {:?}", i),
                },
                TableCell {
                    key: "column4".to_string(),
                    value: format!("Column 4 – Row {:?}", i),
                },
                TableCell {
                    key: "column5".to_string(),
                    value: format!("Column 5 – Row {:?}", i),
                },
            ])
        }).collect::<Vec<TableRow>>();

        html! {
            <Page>
                <h1>{"Table"}</h1>
                <Table selected_index=self.selected headers=headers rows=rows on_select=self.link.callback(|idx| Msg::Select(idx))></Table>
            </Page>
        }
    }
}
