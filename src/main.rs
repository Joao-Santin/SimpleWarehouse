use iced::{Element, Task, widget::{row, column, container, text}};

pub struct Company{
    name: String, 
}

pub enum Screen{
    HubScreen,
    InputScreen,
    OutputScreen,
}

#[derive(Debug)]
pub enum Message{
    ScreenMessage,
    InputChanged,
    ButtonPressed,
}

pub struct WarehouseApp{
    sel_company: Company,
    sel_screen: Screen,
}
impl Default for WarehouseApp{
    fn default() -> Self{
        Self{
            sel_company: Company{name:"unselected".to_string()},
            sel_screen: Screen::HubScreen,
        }
    }
} 

impl WarehouseApp{
    fn title(&self) -> String{
        let comp_title = &self.sel_company.name;
        format!("Simple!Warehouse: {}",comp_title)
    }

    fn update(&mut self, event: Message) -> Task<Message> {
        match event{
            Message::ScreenMessage => {
                println!("ScreenMessage");
                Task::none()
            }
            Message::InputChanged => {
                println!("InputChanged");
                Task::none()
            }
            Message::ButtonPressed => {
                println!("ButtonPressed");
                Task::none()
            } 

        }
    }
    fn view(&self) -> Element<Message> {
       let teste = column![
            row![
                text("Teste simples")
            ]
        ];
        container(teste).into()

    }
}
fn main() -> iced::Result {
    iced::application(WarehouseApp::title, WarehouseApp::update, WarehouseApp::view)
        .default_font(iced::Font::MONOSPACE)
        .centered()
        .run()
}

async fn query_mongo() -> Result<(), String>{
    Ok(())
}
