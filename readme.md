# TFchain monitoring tool

Building:

`cargo build`

## Config file

Config file be formatted like the following:

```yaml
- network:
  network_url: "wss://tfchain.dev.grid.tf"
  accounts:
    - name: Activation Service
      address: 5GNNdXSQJC5LxKv1s2zvhCwyWyCg1msGPanHCsSNbWyu2Z2n
    - name: Bridge
      address: 5Dfiaccytu741LE8KxqegrBeVKgmCKRHU7KEuamf3igxgGW9
    - name: Offchain worker
      address: 5GRiEUaZZxuS7J1VbCvmz9LRdgyMJR9mxPE9JzCZu45JyXTx
```

## Running

```
export TELEGRAM_BOT_TOKEN=token
export TELEGRAM_CHAT_ID=-1231231
cargo run ./config.yaml
```