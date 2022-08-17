pub fn template() {
    let exec_path = std::env::current_exe().expect("Couldn't find the executable's path");
    let service_output = format!(
        r#"[Unit]
Description=UwUssimo Minecraft Server
After=network.target

[Service]
Nice=1
KillMode=none
SuccessExitStatus=0 1
ProtectHome=true
ProtectSystem=full
PrivateDevices=true
NoNewPrivileges=true
WorkingDirectory=/opt/minecraft
ExecStart={} start
ExecStop=/usr/bin/pkill uwucrafter

[Install]
WantedBy=multi-user.target"#,
        exec_path
            .into_os_string()
            .into_string()
            .expect("Couldn't parse the executable's location")
    );
}
