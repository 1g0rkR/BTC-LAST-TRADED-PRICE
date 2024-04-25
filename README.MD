# Test application retrive Bitcoin Last Traded Price (ltp)

## Env variables
Based on `env.example` file make `.env` in the root folder.

- BTC_PAIRS (example: "BTC/USD,BTC/EUR,BTC/CHF")
- PORT_NUMBER (default 8000)

## Run app locally
#### Requirements
- Installed rustup (https://rustup.rs/)

Use the following command to start server locally
`source .env && cargo r --release --bin btc_app`

## Run app in Docker
#### Requirements
- [Docker](https://docs.docker.com/engine/install/)
- [Docker-compose](https://docs.docker.com/compose/install/)

Use the following command to start btc_app:
`sudo docker-compose up --build -d`

Use the following command to shutdown btc_app:
`sudo docker-compose down`

## Run Unit tests
Use the command to run unit tests:
`source .env && cargo t --bin btc_app --tests`