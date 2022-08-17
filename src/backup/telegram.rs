use std::fmt::Write;

const TELEGRAM: &str = "https://api.telegram.org";

pub struct Param {
    title: String,
    key: String,
}

pub struct Telegram {
    token: String,
}

impl Telegram {
    pub fn new(token: &str) -> Telegram {
        Telegram {
            token: token.parse().unwrap(),
        }
    }

    async fn request(&self, method: &str, params: Vec<Param>) {
        let mut base = String::new();

        write!(base, "{}/bot{}/{}", TELEGRAM, &self.token, method)
            .expect("Could not write path to base URL");

        for (index, param) in params.iter().enumerate() {
            write!(
                base,
                "{}{}={}",
                query_delimiter(index),
                param.title,
                param.key
            )
            .expect("Could not write query to base URL");
        }

        println!("{}", base);

        reqwest::get(base).await.expect("Couldn't send the request");
    }

    pub async fn send(&self, chatid: i64, message: &str) {
        self.request(
            "sendMessage",
            vec![
                Param {
                    title: "chat_id".to_string(),
                    key: chatid.to_string(),
                },
                Param {
                    title: "text".to_string(),
                    key: message.to_string(),
                },
                Param {
                    title: "parse_mode".to_string(),
                    key: "HTML".to_string(),
                },
            ],
        )
        .await
    }
}

fn query_delimiter(index: usize) -> &'static str {
    if index == 0 {
        "?"
    } else {
        "&"
    }
}
