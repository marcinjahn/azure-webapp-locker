# API Playground Picklock

This project either opens or closes public access to the API Playground.
Obviously, it's just an example code and it should never be used, because
API Playground should be accessible only via iBoss proxy.

## Usage

```sh
npm install
node index.js 1 # public access
node index.js 0 # access only via iBoss
```
