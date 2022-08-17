use rcon_client::{AuthRequest, RCONClient, RCONConfig, RCONError, RCONRequest};

pub async fn send(address: &str, password: &str, cmd: &str) -> Result<(), RCONError> {
    let mut client = RCONClient::new(RCONConfig {
        url: address.to_string(),
        // Optional
        read_timeout: Some(13),
        write_timeout: Some(37),
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
