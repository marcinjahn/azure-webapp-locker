use std::error::Error;
use std::result::Result;
use azure_web_app_locker::{lock, unlock, AppConfig};

fn main() -> Result<(), Box<dyn Error>> {
    let error_msg =
        "1 or 0 should be provided, where 1 will unlock API Playground and 0 will lock it";
    let param = std::env::args().nth(1).expect(error_msg);

    let config = get_config()?;

    match param.as_str() {
        "0" => lock(config),
        "1" => unlock(config),
        _ => {
            panic!("{}", error_msg)
        }
    }?;

    Ok(())
}

fn get_config() -> Result<AppConfig, Box<dyn Error>> {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("config/config_default.toml"))?
        .merge(config::File::with_name("config/config.toml"))?;
    Ok(settings.try_into::<AppConfig>()?)
}

