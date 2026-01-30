use std::{
    error::Error,
    io::{self},
};

use crate::config::load_tg_client_config;
use grammers_client::SignInError;
use grammers_client::{InvocationError, types::Dialog};
use grammers_mtsender::SenderPool;
use grammers_session::storages::SqliteSession;

use grammers_client::Client;
pub struct ConnectClientReturnType {
    pub client: Client,
}

fn create_sender_pool(session_path: &str, api_id: i32) -> Result<SenderPool, Box<dyn Error>> {
    let session = SqliteSession::open(session_path)?;

    let sender_pool = SenderPool::new(std::sync::Arc::new(session), api_id);

    Ok(sender_pool)
}

pub async fn connect_client(session_path: &str) -> Result<ConnectClientReturnType, Box<dyn Error>> {
    let config = load_tg_client_config()?;

    let sender_pool = create_sender_pool(session_path, config.api_id)?;

    let client = Client::new(&sender_pool);

    tokio::spawn(sender_pool.runner.run());

    if !client.is_authorized().await? {
        let token = client
            .request_login_code(&config.phone_number, config.api_hash.as_str())
            .await?;

        println!("Enter the OTP code: ");
        let mut code = String::new();
        io::stdin().read_line(&mut code)?;
        let code = code.trim();

        match client.sign_in(&token, code).await {
            Ok(_) => println!("Logged in successfully!"),
            Err(SignInError::PasswordRequired(password_token)) => {
                client
                    .check_password(password_token, &config.password)
                    .await?;
            }
            Err(e) => return Err(e.into()),
        }
    }

    println!(
        "Connected to Telegram via {}!",
        client.get_me().await.unwrap().full_name()
    );
    Ok(ConnectClientReturnType { client })
}

pub async fn get_dialogs(client: &Client) -> Result<Vec<Dialog>, InvocationError> {
    let mut iter_dialogs = client.iter_dialogs();

    let dialogs_len = iter_dialogs.total().await.unwrap_or(0);

    let mut dialogs: Vec<Dialog> = vec![];

    for _ in 0..dialogs_len {
        let next_dialog_option = iter_dialogs.next().await?;
        if let Some(next_dialog) = next_dialog_option {
            dialogs.push(next_dialog);
        }
    }

    Ok(dialogs)
}
