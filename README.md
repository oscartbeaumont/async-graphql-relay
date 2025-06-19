<h1 align="center">async-graphql-relay</h1>
<div align="center">
 <strong>
   Relay support for async-graphql
 </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/async-graphql-relay">
    <img src="https://img.shields.io/crates/v/async-graphql-relay.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/async-graphql-relay">
    <img src="https://img.shields.io/crates/d/async-graphql-relay.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/async-graphql-relay">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>
<br/>

This crate aims to bring the [Relay server specification](https://relay.dev/docs/guides/graphql-server-specification) to [async-graphql](https://github.com/async-graphql/async-graphql). The Relay specification requires three main things:
 - Globally unique IDs
    - This crate achieves this by appending the database UUID with an integer which represents the objects type
 -  Refetching using the `node` query
    - This crate calls a `get()` method on the type specified in the globally unique ID to refetch the object.
 - Connections for Pagination
    - This feature already exists in [async-graphql](https://github.com/async-graphql/async-graphql). [Documentation for this feature is here](https://async-graphql.github.io/async-graphql/en/cursor_connections.html).

## Install

Add `async-graphql-relay` to your dependencies:

```toml
[dependencies]
# ...
async-graphql-relay= "0.5.0"
```
## Usage

Check out [this example application](https://github.com/oscartbeaumont/async-graphql-relay/tree/main/example).
