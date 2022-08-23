mod rcon;
mod telegram;

use crate::execute::{output, run};
use rand::Rng;
use telegram::Telegram;

pub async fn upload(token: &str, chat_id: i64) {
    let session = output("git", vec!["status", "--porcelain"]).await;

    if !session.is_empty() {
        println!("Found changes. Backing up!");
        let client = Telegram::new(token);

        println!("Sending notification to Telegram Chat");
        client
            .send(
                chat_id,
                "<b>⚠️ Attention to all players!</b>\nWe are backing up our system! It may take some time...",
            )
            .await;

        // Creating rand instance
        let mut rng = rand::thread_rng();

        // Stop the server
        run("sudo", vec!["systemctl", "stop", "minecraft.service"]).await;

        // Start backing process
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
