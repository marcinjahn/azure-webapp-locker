# API Playground Picklock

This application either opens or closes public access to the API Playground.
Obviously, it's just an example code and it should never be used, because
API Playground should be accessible only via iBoss proxy.

## Usage

To run the app, there needs to be config file `config/config.toml`.
Thhe contents of that file should be based on the contents of `config/config_default.toml`:

```toml
subscription_id = ""
rg_name = ""
web_app_name = ""
web_app_location = ""
client_id = ""
client_secret = ""
tenant_id = ""
rule_priority_number = 444
```

When the config is ready, you can run the app as follows (make sure Rust is installed):

```sh
cargo run --quiet -- 1 # unlock public access
cargo run --quiet -- 0 # lock access only via iBoss
```
