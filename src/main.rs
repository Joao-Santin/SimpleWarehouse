use iced::{Element, widget::{row, column, button, container, text}};

pub enum Company{
    RSJ,
    BSR
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
    ButtonPressed
}

pub struct WarehouseApp{
    sel_company: Company,
    sel_screen: Screen,
}
impl Default for WarehouseApp{
    fn default() -> Self{
        Self{
            sel_company: Company::RSJ,
            sel_screen: Screen::HubScreen,
        }
    }
} 
impl WarehouseApp{
    fn title(&self) -> String{
        let comp_title: String = match self.sel_company{
            Company::RSJ => "RSJ".to_string(),
            Company::BSR => "BSR".to_string()
        };
        format!("Simple!Warehouse: {}",comp_title )
    }
    fn update(&mut self, event: Message) {
        match event{
            Message::ScreenMessage => println!("ScreenMessage"),
            Message::InputChanged => println!("InputChanged"),
            Message::ButtonPressed => println!("ButtonPressed"),

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
