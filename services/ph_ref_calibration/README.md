# ph ref calibration

You must configure Rocket.toml manually. Here is an example:

```toml
[global]
address = "0.0.0.0"        # address for the web server
namespace = "shrimpfiesta" # data namespace used for redis interaction

[global.databases]
redis = { url = "redis://yourhost:6379" }
```

[Read some docs](https://rocket.rs/v0.4/guide/state/#usage)
