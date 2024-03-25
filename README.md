# Tsunami DB

archived: until I have an attack on this non-trivial big problem.

> If you do not work on an important problem, it's unlikely you'll do important work. It's perfectly obvious. Great scientists have thought through, in a careful way, a number of important problems in their field, and they keep an eye on wondering how to attack them. Let me warn you, "important problem" must be phrased carefully. The three outstanding problems in physics, in a certain sense, were never worked on while I was at Bell Labs. By important I mean guaranteed a Nobel Prize and any sum of money you want to mention. We didn't work on (1) time travel, (2) teleportation, and (3) antigravity. They are not important problems because we do not have an attack. It's not the consequence that makes a problem important, it is that you have a reasonable attack. That is what makes a problem important.

An alternative (`:ets` drop-in replacement)  BEAM compatible key/value store

# Goals
- `:ets` api/behaviour compatibility
-  can expose a runtime api for [firefly](https://github.com/GetFirefly/firefly) & erlang/otp
- exposes a NIF api

## Non-Goals/Out of Scope
- `:dets`
- replication or any form of transaction support
