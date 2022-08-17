use crate::execute::output;

pub async fn upload() {
    let session = output("git", vec!["status", "--porcelain"]).await;

    if !session.is_empty() {
        println!("{}, {}", session, session.len());
    } else {
        println!("Sorry for that...");
    }
}