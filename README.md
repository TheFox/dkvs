# dkvs

Distributed Key-Value Storage written in Rust.

## Dev

```bash
lsof -Pnp $(ps -ef | grep dkvs_server | grep -v grep | awk '{ print $2 }')
```
