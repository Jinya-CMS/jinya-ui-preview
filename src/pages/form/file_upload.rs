use jinya_ui::layout::page::*;
use jinya_ui::widgets::form::file_upload::*;
use yew::format::Json;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::services::reader::File;
use jinya_ui::layout::form::*;

pub struct FileUploads {
    link: ComponentLink<Self>,
    default_value: Vec<File>,
    positive_value: Vec<File>,
    negative_value: Vec<File>,
    negative_validation_message: String,
}

pub enum Msg {
    DefaultInput(Vec<File>),
    PositiveInput(Vec<File>),
    NegativeInput(Vec<File>),
}

impl Component for FileUploads {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        FileUploads {
            link,
            default_value: vec![],
            positive_value: vec![],
            negative_value: vec![],
            negative_validation_message: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::DefaultInput(value) => {
                self.default_value = value.clone();
                for file in value {
                    ConsoleService::log(format!("{}", file.name()).as_str());
                }
            }
            Msg::PositiveInput(value) => self.positive_value = value,
            Msg::NegativeInput(value) => {
                self.negative_value = value;
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
                <h1>{"Text fields"}</h1>
                <Form>
                    <FileUpload multiple=true on_select=self.link.callback(|files| Msg::DefaultInput(files)) placeholder="Default placeholder" label="Default upload" filename="Random file" />
                </Form>
            </Page>
        }
    }
}
