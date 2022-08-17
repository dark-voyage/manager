use crate::execute;

pub async fn start() {
    execute::run(
        "java",
        vec!["-Xmx5120M", "-Xms1024M", "-jar", "server.jar", "nogui"],
    )
    .await
}
