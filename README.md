# remoteit-api-wrapper-rs

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

Once you have the credentials, you have two options to provide them to the library:

### Providing credentials using the credentials file as per remote.it spec

:warning: You need to enable the `credentials_loader` feature to use this method.
You can do this by running `$ cargo add -F credentials_loader`, or editing your `Cargo.toml` to look like this:
```toml
...
[dependencies]
remoteit-api = { version = "...", features = ["credentials_loader"] }
...
```

Save your remote.it credentials to `~/.remoteit/credentials`.  
The file should look like this:
```ini
[default]
R3_ACCESS_KEY_ID=
R3_SECRET_ACCESS_KEY=
```




