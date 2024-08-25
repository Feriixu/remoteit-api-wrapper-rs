# remoteit-api

[![Crates.io](https://img.shields.io/crates/v/remoteit-api)](https://crates.io/crates/remoteit-api)
[![License](https://img.shields.io/crates/l/remoteit-api)](https://crates.io/crates/remoteit-api)
[![Docs.rs](https://docs.rs/remoteit-api/badge.svg)](https://docs.rs/remoteit-api)

This is a Rust wrapper for the [Remote.it API](https://docs.remote.it/developer-tools/api).

Remote.it is a service that allows you to connect to devices remotely. It provides a way to connect to devices behind NATs and firewalls without having to configure port forwarding.

## Blocking or Async

This library provides both blocking and async versions of the API.
Neither is enabled by default though, so you need to choose one of them by enabling the corresponding feature.
```shell
cargo add remoteit-api -F blocking
cargo add remoteit-api -F async
```

Which one you want to use depends on your use case. If you are writing a CLI tool or a small script, you might want to use the blocking version.
If you are writing a server or a GUI application, you might want to use the async version.

The developer API is pretty much the same for both versions, so you can switch between them easily.

## Providing Credentials

You can get your credentials here: https://app.remote.it/#/account/accessKey.  
_Note: You need to have a Remote.it account to get the credentials._

Once you have the credentials, you have several options to use them with this crate:

### Providing credentials using the credentials file as per remote.it spec

ℹ️ _You need to enable the `credentials_loader` feature to use this method.
You can do this by running `$ cargo add -F credentials_loader`, or editing your `Cargo.toml` to look like this:_
```toml
# ...
[dependencies]
remoteit-api = { version = "...", features = ["credentials_loader"] }
# ...
```

Save your remote.it credentials to `~/.remoteit/credentials`. The file should look like this:
```ini
[default]
R3_ACCESS_KEY_ID=
R3_SECRET_ACCESS_KEY=
```

Load the credentials using the `Credentials::load_from_disk` function.  
When calling this function, you have the option of providing a custom file path.

### Providing credentials directly

If you want to store the credentials in a different way, you can also provide them directly by using the `Credentials::builder()` function.
You do not need to enable any features for this method.

## Developing your own GraphQL operations

### Getting the schema

If the operations defined in this crate do not fit your use-case, you must write your own GraphQL queries.
However, obtaining the schema of the remote.it API is a bit tricky, as all requests must be signed using remote.it's custom signing algorithm.

To get the schema, you can simply copy the [schema.json](src/graphql/schema.json).  

If you want to query the newest schema, it is recommended to use [Insomnia](https://insomnia.rest/).
Follow the instructions from the remote.it docs on how to setup Instomnia: https://docs.remote.it/developer-tools/api/using-developer-tools.  
Then, add a new requests to the Insomnia Collection and paste the body from [InstrospectSchema.graphql](src/graphql/IntrospectSchema.graphql).
The result should be the same as [schema.json](src/graphql/schema.json).

### Writing the query

Please follow the examples in [src/graphql](src/graphql) on how to write queries.

### Using the query

You can follow the examples in [operations.rs](src/operations.rs) for guidance on how to use the `graphql_client` crate.
See [api_blocking.rs](src/api_blocking.rs) or [api_async.rs](src/api_async.rs) for example on how to use the code, generated by `graphql_client`.

To sign and send the requests, you can use `R3Client::send_remoteit_graphql_request` from [api_blocking.rs](src/api_blocking.rs) or `R3Client::send_remoteit_graphql_request_async` from [api_async.rs](src/api_async.rs)
or implement your own signing and sending function, following the example of aforementioned functions.



## Note on reqwest and TLS

This crate uses `reqwest` to make HTTP requests.
You may want to enable the `native-tls-vendored` feature to use the vendored version of `native-tls` with `reqwest`.
This is mainly useful for cross-compiling. If you are not cross-compiling, this will most likely be unnecessary.