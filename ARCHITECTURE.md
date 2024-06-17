# why?

This is basically a document that explains at a high-level my reasoning why this might be a good idea.

## Motivation, why re-write it in rust?

- What is ets at its storage core? it is two data structures a hash map, and a tree. The tree is basically an AVL Tree + a CA Tree for the ordered api and the hashmap for the unordered api, this design however was cutting edge research in 2017, _a lot has changed_ in the world of efficient concurrent ranked-balance search trees. It is possible to find an alternative with desirable tradeoffs.

- Its also acquired alot of bit rot, tight integration into the rest of the runtime has introduced alot of... stuff. baggage.. The C source.. is unweidly and difficult to understand or I'm dumb or both.

- The match spec query syntax is old, and kinda weird. Native support for dataframes, sql or extensions to support custom query languages. A lazy modern ergonomic api should exist. Explorer via polars and other engines bring this to the elixir/erlang ecosystem but it is not coupled with an in-memory storage engine, it is as the name suggests for "exploring" large data sets on local or remote block storage via `ObjectStorage`.

- Arrow as the zero copy in-memory data-format for/as Terms could be more efficient as a default.

- Leveraging the Apache Datafusion and rust ecosystem of crates like dashmap, cross-beam and parking lot might lead to unforseen gains in performance + safety.

- Even with the overhead of an eventual NIF api this could be competitively performant.

- Why not an embedded vector database? ai is all the rage?

