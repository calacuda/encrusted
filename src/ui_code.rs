use std::boxed::Box;
use std::ffi::CString;

use traits::UI;

#[derive(Debug, Clone)]
enum Token {
    Newline,
    Text(String),
    Object(String),
    Debug(String),
}

#[derive(Debug)]
pub struct CodeUI {
    buffer: Vec<Token>,
}

impl UI for CodeUI {
    fn new() -> Box<CodeUI> {
        Box::new(CodeUI { buffer: Vec::new() })
    }

    fn print(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }

        if text == "\n" {
            self.buffer.push(Token::Newline);
            return;
        }

        if !text.contains('\n') {
            self.buffer.push(Token::Text(String::from(text)));
            return;
        }

        let lines = text.lines().collect::<Vec<_>>();

        for (index, line) in lines.iter().enumerate() {
            if !line.is_empty() {
                self.buffer.push(Token::Text(String::from(*line)));
            }

            if let Some(_) = lines.get(index + 1) {
                self.buffer.push(Token::Newline);
            }
        }

        if text.ends_with('\n') {
            self.buffer.push(Token::Newline);
        }
    }

    fn debug(&mut self, text: &str) {
        self.buffer.push(Token::Debug(String::from(text)));
    }

    fn print_object(&mut self, obj: &str) {
        self.buffer.push(Token::Object(String::from(obj)));
    }

    fn flush(&mut self) {
        // TODO: make print to terminal

        // if self.buffer.is_empty() {
        //     return;
        // }
        //
        // let mut html = String::new();
        //
        // for (index, item) in self.buffer.iter().enumerate() {
        //     let prev = if index == 0 {
        //         None
        //     } else {
        //         self.buffer.get(index - 1)
        //     };
        //
        //     let next = self.buffer.get(index + 1);
        //
        //     match *item {
        //         Token::Newline => {
        //             html.push_str("<br>");
        //         }
        //         Token::Text(ref text) => {
        //             match prev {
        //                 Some(&Token::Text(_)) => (),
        //                 _ => html.push_str("<span>"),
        //             }
        //
        //             html.push_str(&text);
        //
        //             match next {
        //                 Some(&Token::Text(_)) => (),
        //                 _ => html.push_str("</span>"),
        //             }
        //         }
        //         Token::Object(ref obj) => {
        //             let class = match (prev, next) {
        //                 (None, Some(&Token::Newline)) => "room",
        //                 (Some(&Token::Newline), Some(&Token::Newline)) => "room",
        //                 _ => "object",
        //             };
        //
        //             write!(html, r#"<span class="{}">{}</span>"#, class, obj).unwrap();
        //         }
        //         Token::Debug(ref text) => {
        //             write!(html, r#"<span class="debug">{}</span>"#, text).unwrap();
        //         }
        //     }
        // }
        //
        // self.message("print", &html);
        // self.buffer.clear();
    }

    fn get_text(&mut self) -> String {

        let buf: Vec<String> = self.buffer.clone().into_iter().enumerate().map(|(index, item)| {
            let prev = if index == 0 {
                None
            } else {
                self.buffer.get(index - 1)
            };

            let next = self.buffer.get(index + 1);

            match item {
                Token::Newline => "\n".to_string(), 
                Token::Text(text) => text,
                Token::Object(obj) => {
                    let class = match (prev, next) {
                        (None, Some(&Token::Newline)) => "room",
                        (Some(&Token::Newline), Some(&Token::Newline)) => "room",
                        _ => "object",
                    };

                    format!("{}\n{}\n", class, obj)
                }
                Token::Debug(text) => text,
            }
        }).collect();
        
        self.buffer.clear();
        buf.join("")
    }


    fn set_status_bar(&self, left: &str, right: &str) {
        let msg = serde_json::to_string(&(left, right)).unwrap();
        self.message("header", &msg)
    }

    fn message(&self, _mtype: &str, _msg: &str) {}

    fn clear(&self) {}
    fn reset(&self) {}
    fn get_user_input(&self) -> String {
        unimplemented!();
    }
}
