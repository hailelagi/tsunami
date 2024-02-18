# Tsunami DB

An alternative (`:ets` drop-in replacement)  BEAM compatible key/value store

# Goals
- `:ets` api/behaviour compatibility
-  can expose a runtime api for [firefly](https://github.com/GetFirefly/firefly) & erlang/otp
- exposes a NIF api

## Non-Goals/Out of Scope
- `:dets`
- replication or any form of transaction support
