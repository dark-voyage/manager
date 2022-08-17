use crate::execute::{output, run};
use crate::telegram::Telegram;
use rand::Rng;

pub async fn upload() {
    let session = output("git", vec!["status", "--porcelain"]).await;

    if !session.is_empty() {
        let client = Telegram::new("5402173317:AAFnj8nUO-PnQsOCYYaoYsMeU6lYcqxuHlI");

        println!("Sending notification to Telegram Chat");
        client
            .send(
                756870298,
                "We are backing up our system! It may take some time...",
            )
            .await;

        println!("Found changes. Backing up!");
        let mut rng = rand::thread_rng();

        run("git", vec!["add", "."]).await;
        run(
            "git",
            vec![
                "commit",
                "-m",
                format!("Sync from local to remote {}", rng.gen::<u32>()).as_str(),
            ],
        )
        .await;
    } else {
        println!("No changes found... Skipping!");
    }
}
