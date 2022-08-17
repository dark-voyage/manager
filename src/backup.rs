use crate::execute::output;

pub async fn upload() {
    let session = output("git", vec!["status", "--porcelain"]).await;

    println!("{}, {}", session, session.len());
}