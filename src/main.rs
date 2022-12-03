extern crate iced;
extern crate regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use iced::widget::{row, column, text_input, button, text};
use iced::{
    window,  executor,
    Application, Command, Element, Settings,
    Theme, Alignment, Length
};

// use iced::theme::Button::Custom as tbutton_custom;

// mod styles;

// use styles::Btn as BtnStyle;

const APP_WIDTH: u32 = 500;
const APP_HEIGHT: u32 = 500;

pub fn main() -> iced::Result {
    // env_logger::builder().format_timestamp(None).init();
    CookieApp::run(Settings {
        antialiasing: true,
        window: window::Settings {
            size: (APP_WIDTH, APP_HEIGHT),
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        default_text_size: 14,
        ..Settings::default()
    })
}

struct CookieApp {
    keys_str: String,
    keys: Vec<String>,
    cookie_keys_str: String,
    cookie_keys: Vec<String>,
    raw_cookies: String,
    account_name: String,
    save_path: String,
    cookies: HashMap<String, Vec<Vec<String>>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputKeyStr(String),
    InputCookieKeyStr(String),
    InputCookie(String),
    InputAccountName(String),
    InputSavePath(String),
    AppendCookie,
    SaveFile,
    AppendKey,
    RemoveKey,
    AppendCookieKey,
    RemoveCookieKey,
}

impl Application for CookieApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    // Инициализируется состояние приложения
    // TODO попробовать поменять значение Command::none
    fn new (_flags: ()) -> (Self, Command<Message>) {
        (
            CookieApp {
                keys_str: String::new(),
                keys: vec![],
                cookie_keys_str: String::new(),
                cookie_keys: vec![],
                raw_cookies: String::new(),
                save_path: env::current_dir().unwrap().display().to_string(),
                account_name: String::new(),
                cookies: HashMap::new(),
            },
            Command::none()
        )
    }
    fn title(&self) -> String {
        String::from("Cookie creator")
    }

    fn update(&mut self, m: Message) -> Command<Message> {
        match m {
            Message::InputAccountName(m) => self.account_name = m,
            Message::InputCookie(m) => self.raw_cookies = m,
            Message::InputKeyStr(m) => self.keys_str = m,
            Message::InputCookieKeyStr(m) => self.cookie_keys_str = m,
            Message::InputSavePath(m) => self.save_path = m,
            Message::AppendCookie => {
                if !self.raw_cookies.is_empty()
                    && !self.account_name.is_empty()
                    && self.cookie_keys.len() > 0 {
                    let raw_cookie = &self.raw_cookies;
                    let re = regex::Regex::new(r"-H '(.+?):\s(.+?)'").unwrap();
                    let cookies: Vec<Vec<String>> = re.captures_iter(raw_cookie)
                        .filter(|item| {
                            let key = item[1].to_string();
                            self.cookie_keys.contains(&key)
                        })
                        .enumerate()
                        .map(|(i, item)| {
                            let key = item[1].to_string();
                            let value = item[2]
                                .to_string()
                                .replace("\"", "\'")
                                .replace("\\", "\\\\");

                            let setting_key_index = self.cookie_keys.iter().position(|cookie_key| *cookie_key == key);

                            if let cookie_index = setting_key_index.unwrap()  {
                                vec![self.keys[cookie_index].clone(), value]
                            } else {
                                vec![key, value]
                            }
                        }).collect();
                    self.cookies.insert(
                        self.account_name.clone(),
                        cookies
                    );
                    self.account_name = "".to_string();
                    self.raw_cookies = "".to_string();
                    println!("{:?}", self.cookies)
                }
            },
            Message::AppendKey => {
                self.keys.push(self.keys_str.clone());
                self.keys_str = "".to_string();
            },
            Message::AppendCookieKey => {
                self.cookie_keys.push(self.cookie_keys_str.clone());
                self.cookie_keys_str = "".to_string();
            },
            Message::RemoveKey => {
                if !self.keys_str.is_empty() {
                    self.keys = self.keys
                        .clone()
                        .into_iter()
                        .filter(|item| *item != self.keys_str)
                        .collect();
                    self.keys_str = "".to_string();
                }
            },
            Message::RemoveCookieKey => {
                if !self.cookie_keys_str.is_empty() {
                    self.cookie_keys = self.cookie_keys
                        .clone()
                        .into_iter()
                        .filter(|item| *item != self.cookie_keys_str)
                        .collect();
                    self.cookie_keys_str = "".to_string();
                }
            },
            Message::SaveFile => {
                let content: String = self.cookies
                    .iter()
                    .map(|(key, value)| {
                        let json_body = value
                            .into_iter()
                            .map(|item| {
                                let key = &item[0];
                                let value = &item[1];
                                format!("\"{}\": \"{}\"", key, value)
                            })
                            .collect::<Vec<String>>()
                            .join(", ");
                        return format!("\"{}\": {{ {} }}", key, json_body);
                    })
                    .collect::<Vec<String>>()
                    .join(", ");
                let file_content = format!("{{{}}}", content);

                let file_creating_result = fs::write(&self.save_path, file_content.as_bytes());
                match file_creating_result {
                    Ok(()) => {
                        println!("Success")
                    },
                    Err(err) => {
                        println!("{:?}", err);
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let top_row_spacing = 10;
        let all_cookie_keys = self.cookie_keys.join(", ");
        let all_json_keys = self.keys.join(", ");
        let appended_accounts = self.cookies
            .keys()
            .cloned()
            .into_iter()
            .collect::<Vec<String>>()
            .join(", ");

        column![
            // Поле для ввода ключей из CURL
            row![
                text_input("Key from cookie", &self.cookie_keys_str, Message::InputCookieKeyStr),
                button("Add").on_press(Message::AppendCookieKey).padding([5, 40]),
                button("Remove")
                    .on_press(Message::RemoveCookieKey)
                    .padding([5, 40])
                    .style(iced::theme::Button::Destructive),
            ].spacing(top_row_spacing),
            text(format!("Selected keys: {}", all_cookie_keys)).width(Length::FillPortion(100)),
            // Поле для ввода ключей в JSON файл
            row![
                text_input("Key for json file", &self.keys_str, Message::InputKeyStr),
                button("Add").on_press(Message::AppendKey).padding([5, 40]),
                button("Remove")
                    .on_press(Message::RemoveKey)
                    .padding([5, 40])
                    .style(iced::theme::Button::Destructive),
            ].spacing(top_row_spacing),

            // Вывод полей для файла json
            text(format!("Selected keys: {}", all_json_keys)).width(Length::FillPortion(100)),
            // Поле для ввода назвнаия аккаунта
            text_input("Account name", &self.account_name, Message::InputAccountName),
            // Поле для ввода куки из браузера
            text_input("Insert cookie as CURL", &self.raw_cookies, Message::InputCookie),

            // Поле для ввода пути сохранения файла
            text_input("Insert path to file", &self.save_path, Message::InputSavePath),

            // Кнопка конвертирования
            row![
                button("Convert")
                    .padding([5, 40])
                    .on_press(Message::AppendCookie),
                button("Save")
                    .padding([5, 50])
                    .on_press(Message::SaveFile)
                    .style(iced::theme::Button::Destructive)
            ].spacing(20),
            text(format!("Appended accounts: {}", appended_accounts)).width(Length::FillPortion(100)),
        ]
        .padding(50)
        .align_items(Alignment::Center)
        .spacing(20)
        .into()

    }
    fn theme(&self) -> Theme {
        Theme::Dark
    }

}

