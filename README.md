# API Playground Picklock

This application either opens or closes public access to the API Playground.
Obviously, it's just an example code and it should never be used, because
API Playground should be accessible only via iBoss proxy.

## Usage

```sh
cargo run --quiet -- 1 # unlock public access
cargo run --quiet -- 0 # lock access only via iBoss
```
