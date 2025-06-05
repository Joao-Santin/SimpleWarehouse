use iced::{Element, executor, Task, widget::{row, column, container, text, button}};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize,Deserialize)]
pub enum CompanyType{
    None,
    Me,
    Customer,
    Supplier
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Company{
    idd: String,
    name: String, 
    itype: CompanyType,
    cnpj: String,
}

impl Default for Company{
    fn default() -> Self{
        Self{
            idd: String::new(),
            name: String::new(),
            itype: CompanyType::None,
            cnpj: String::new()
        }
    }
    
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Item{
    idd: String,
    itype: String,
}

pub enum Screen{
    HubScreen,
    InputScreen,
    OutputScreen,
}

#[derive(Debug, Clone)]
pub enum Message{
    ScreenMessage,
    InputChanged,
    ButtonPressed,
    MongoResult(Result<(), String>)
}

pub struct WarehouseApp{
    sel_company: Company,
    sel_screen: Screen,
}
impl Default for WarehouseApp{
    fn default() -> Self{
        Self{
            sel_company: Company::default(),
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
                println!("Consultando mongodb...");
                Task::perform(query_mongo(), Message::MongoResult)
            } 
            Message::MongoResult(Ok(())) => {
                println!("Consulta Mongodb feita com sucesso!");
                Task::none()
            }
            Message::MongoResult(Err(e)) => {
                println!("Consulta Mongodb falhou: {}", e);
                Task::none()
            }

        }
    }
    fn view(&self) -> Element<Message> {
       let teste = column![
            row![
                text("Teste simples"),
                button(text("Testing mongodb!")).on_press(Message::ButtonPressed)
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
    use mongodb::{bson::{doc, to_document, Document}, options::ClientOptions, Client};
    use std::env;

    dotenv::dotenv().ok();
    let uri = env::var("MONGOURI").expect("You must set your MONGOURI in your .env file.");
    let options = ClientOptions::parse(&uri).await.map_err(|e| e.to_string())?;
    let client = Client::with_options(options).map_err(|e| e.to_string())?;

    let db = client.database("simple_warehouse");
    let col = db.collection::<mongodb::bson::Document>("empresa");

    col.insert_one(doc! {"testing": "Testing app"})
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
