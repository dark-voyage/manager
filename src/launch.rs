use crate::execute;

pub async fn start() {
    execute::run("java", ["-Xmx1024M", "-Xms1024M", "-jar", "server.jar", "nogui"]).await
}
