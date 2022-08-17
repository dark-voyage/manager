use crate::execute::{output, run};
use rand::Rng;

pub async fn upload() {
    let session = output("git", vec!["status", "--porcelain"]).await;

    if !session.is_empty() {
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
        run("git", vec!["push"]).await;
    } else {
        println!("No changes found... Skipping!");
    }
}
