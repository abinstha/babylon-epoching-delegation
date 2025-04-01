## Store WASM Contract

```bash
babylond tx wasm store ./artifacts/babylon_epoching_lib.wasm \
  --from baby_wallet \
  --chain-id bbn-test-5 \
  --gas 10000000 \
  --fees=30000ubbn \
  --node https://rpc-babylon-testnet.imperator.co \
  -y -b sync -o json
```

## Instantiate WASM Contract

```bash
babylond tx wasm instantiate 353 '{}' \
  --from=baby_wallet \
  --admin=baby_wallet \
  --label=test-label \
  --gas=2000000 \
  --fees=10000ubbn \
  --chain-id=bbn-test-5 \
  -b=sync -y \
  --log_format=json -o json \
  --node https://rpc-babylon-testnet.imperator.co
```

## Staking/Delegation

```bash
babylond tx wasm execute bbn1qya74zsq3yc3xx0fxq6elwyj7xx3pv30heyc6kz9qt7rl3wjff0qhk4qhg \
  '{"delegate" : {}}' \
  --amount 1000000ubbn \
  --from=baby_wallet \
  --chain-id="bbn-test-5" \
  --yes \
  --fees=10000ubbn \
  --gas=2000000 \
  --node=https://rpc-babylon-testnet.imperator.co
```

You can find the error txn here: https://testnet.babylon.explorers.guru/transaction/1CDEB943BE9B5AAC987D6C816CBC81990C7C7EEBC00DDEEB06FB1BECEAD58D96?height=643990
