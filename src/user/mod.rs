use serde::Deserialize;

use account::Account;
use settings::Settings;

mod account;
pub mod settings;

pub struct User {
    pub data: UserData,
    pub account: Account,
    pub settings: Settings,
}

#[derive(Debug, Deserialize)]
pub struct UserResponse {
    result: UserData,
}

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub email: String,
}

impl User {
    pub fn new() -> Result<User, failure::Error> {
        let settings = Settings::new()?;

        let account = Account::new(settings.clone())?;
        let data = data(settings.clone())?;
        Ok(User {
            settings,
            account,
            data,
        })
    }
}

fn data(settings: Settings) -> Result<UserData, failure::Error> {
    let user_addr = "https://api.cloudflare.com/client/v4/user";

    let client = reqwest::Client::new();

    let mut res = client
        .get(user_addr)
        .header("X-Auth-Key", settings.global_user.api_key)
        .header("X-Auth-Email", settings.global_user.email)
        .send()?;

    let user_res: UserResponse = serde_json::from_str(&res.text()?)?;
    let user: UserData = user_res.result;
    Ok(user)
}
