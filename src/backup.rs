use crate::execute::{output, run};
use crate::telegram::Telegram;
use rand::Rng;

pub async fn upload(token: &str, chat_id: i64) {
    let session = output("git", vec!["status", "--porcelain"]).await;

    if !session.is_empty() {
        let client = Telegram::new(token);

        println!("Sending notification to Telegram Chat");
        client
            .send(
                chat_id,
                "<b>⚠️ Attention to all players!</b>\nWe are backing up our system! It may take some time...",
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
