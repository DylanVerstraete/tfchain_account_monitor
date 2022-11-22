# TFchain monitoring tool

Building:

`cargo build`

## Config file

Config file be formatted like the following:

```yaml
- network:
  network_url: "wss://tfchain.dev.grid.tf:433"
  accounts:
    - name: Activation Service
      address: 5GNNdXSQJC5LxKv1s2zvhCwyWyCg1msGPanHCsSNbWyu2Z2n
    - name: Bridge
      address: 5Dfiaccytu741LE8KxqegrBeVKgmCKRHU7KEuamf3igxgGW9
    - name: Offchain worker
      address: 5GRiEUaZZxuS7J1VbCvmz9LRdgyMJR9mxPE9JzCZu45JyXTx
```

Note: url should have a port, in case of `wss` it's 443 otherwise it could be `9944` or `80` depending on the settings of the websocket endpoint.

## Running

```
export TELEGRAM_BOT_TOKEN=token
export TELEGRAM_CHAT_ID=-1231231
cargo run ./config.yaml
```