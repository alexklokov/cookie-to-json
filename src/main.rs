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


const APP_WIDTH: u32 = 500;
const APP_HEIGHT: u32 = 500;

pub fn main() -> iced::Result {
    CookieApp::run(Settings {
        antialiasing: true,
        window: window::Settings {
            size: (APP_WIDTH, APP_HEIGHT),
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        default_font: Some(include_bytes!("../font/Menlo-Regular.ttf")),
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
                cookie_keys_str: String::new(),
                cookie_keys: vec![
                    "cookie".to_string(),
                    "x-csrftoken".to_string(),
                    "x-ig-www-claim".to_string()
                ],
                keys_str: String::new(),
                keys: vec![
                    "Cookie".to_string(),
                    "X-CSRFToken".to_string(),
                    "X-IG-WWW-Claim".to_string()
                ],

                raw_cookies: String::new(),
                save_path: env::current_dir().unwrap().display().to_string(),
                account_name: String::new(),
                cookies: HashMap::new(),
            },
            Command::none()
        )
    }
    fn title(&self) -> String {
        String::from("Преобразователь CUrl в JSON")
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
                    let re = regex::Regex::new(r#"-H ['|"](.+?):\s(.+?)['|"][\s|\n]"#).unwrap();
                    let cookies: Vec<Vec<String>> = re.captures_iter(raw_cookie)
                        .filter(|item| {
                            println!("{:?}", item);
                            let key = item[1].to_string().to_lowercase();
                            self.cookie_keys.contains(&key)
                        })
                        .enumerate()
                        .map(|(_, item)| {
                            let key = item[1].to_string().to_lowercase();
                            let value = item[2]
                                .to_string()
                                .replace("\"", "\'")
                                .replace("\\", "\\\\");

                            let setting_key_index = self.cookie_keys
                                .iter()
                                .map(|item| item.to_lowercase().clone())
                                .position(|cookie_key| *cookie_key == key);

                            return match setting_key_index {
                                Some(cookie_index) => vec![self.keys[cookie_index].clone(), value],
                                None => vec![key, value]
                            };
                        }).collect();
                    self.cookies.insert(
                        self.account_name.clone(),
                        cookies
                    );
                    self.account_name = "".to_string();
                    self.raw_cookies = "".to_string();
                }
            },
            Message::AppendKey => {
                self.keys.push(self.keys_str.clone());
                self.keys_str = "".to_string();
            },
            Message::AppendCookieKey => {
                self.cookie_keys.push(self.cookie_keys_str.to_lowercase().clone());
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
                text_input("Ключи из CUrl", &self.cookie_keys_str, Message::InputCookieKeyStr),
                button("Добавить").on_press(Message::AppendCookieKey).padding([5, 40]),
                button("Удалить")
                    .on_press(Message::RemoveCookieKey)
                    .padding([5, 40])
                    .style(iced::theme::Button::Destructive),
            ].spacing(top_row_spacing),
            text(format!("Выбранные ключи: {}", all_cookie_keys)).width(Length::FillPortion(100)),
            // Поле для ввода ключей в JSON файл
            row![
                text_input("Ключи для файла", &self.keys_str, Message::InputKeyStr),
                button("Добавить").on_press(Message::AppendKey).padding([5, 40]),
                button("Удалить")
                    .on_press(Message::RemoveKey)
                    .padding([5, 40])
                    .style(iced::theme::Button::Destructive),
            ].spacing(top_row_spacing),

            // Вывод полей для файла json
            text(format!("Выбранные ключи: {}", all_json_keys)).width(Length::FillPortion(100)),
            // Поле для ввода назвнаия аккаунта
            text_input("Название аккаунта", &self.account_name, Message::InputAccountName),
            // Поле для ввода куки из браузера
            text_input("Вставьте строку CUrl", &self.raw_cookies, Message::InputCookie),

            // Поле для ввода пути сохранения файла
            text_input("Вставьте путь к сохраняемому файлу", &self.save_path, Message::InputSavePath),

            // Кнопка конвертирования
            row![
                button("Преобразовать")
                    .padding([5, 40])
                    .on_press(Message::AppendCookie),
                button("Соханить")
                    .padding([5, 50])
                    .on_press(Message::SaveFile)
                    .style(iced::theme::Button::Destructive)
            ].spacing(20),
            text(format!("Преобразованные аккаунты: {}", appended_accounts)).width(Length::FillPortion(100)),
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

