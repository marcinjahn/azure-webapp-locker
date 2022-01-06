# Azure Web App Locker

This application either opens or closes public access to selected Azure Web App.
The app assumes that the app is restricted from public access by default. it has 
two modes:

- unlock
- lock

The UNLOCK mode adds a rule to the web app's firewall that allows in all the traffic from
Internet. The rule will have a priority number as specified in the config file.
The LOCK mode removes the rule with a specified priority number.

## Usage

To run the app, there needs to be a config file `config/config.toml`.
The contents of that file should be based on the contents of `config/config_default.toml`:

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
