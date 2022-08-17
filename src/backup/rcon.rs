use rcon_client::{AuthRequest, RCONClient, RCONConfig, RCONError, RCONRequest};

pub async fn _send(address: Option<&str>, password: &str, cmd: &str) -> Result<(), RCONError> {
    let mut client = RCONClient::new(RCONConfig {
        url: match address {
            None => "localhost:25575".to_string(),
            Some(adr) => adr.to_string(),
        },
        read_timeout: None,
        write_timeout: None,
    })?;

    let auth_result = client.auth(AuthRequest::new(password.to_string()))?;

    match auth_result.is_success() {
        true => {
            client.execute(RCONRequest::new(cmd.to_string()))?;
        }
        false => {}
    }

    Ok(())
}
