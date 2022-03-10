# minecrevy_config

Provides the `Config` struct used to configure a Minecrevy server.

## Server Owners

Edit the `config.toml`.

### Environment Variables

- `MINECREVY_CONFIG_PATH` (optional): The path to the config file that should be used.

## Server Developers

The config is accessible as a bevy resource: `Res<Config>` (or `ResMut<Config>`).
This functionality is provided through the `ConfigPlugin`.

```rust
fn my_system(config: Res<Config>) {
    // read from the config
}
```
