My adventures in Luca Palmieri's [Zero To Production In Rust](https://www.zero2prod.com).

Make sure you have the following installed:
- OpenSSL
- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker) (might need to follow up with `systemctl start docker`)
- [psql command-line client](https://blog.timescale.com/blog/how-to-install-psql-on-mac-ubuntu-debian-windows)
- sqlx-cli ([instructions here](scripts/init_db.sh))
- LLVM stuff ([instructions here](.cargo/config.toml))

To start up the PostgreSQL and Redis databases in a detached Docker container:
```
./scripts/init_db.sh
./scripts/init_redis.sh
```

Once the databases are running, it's possible to build, run or test in the local environment with `cargo build`, `cargo run` or `cargo test`, respectively.

Once the database is running, it's also possible to statically prepare the binary's SQL queries. This repo already includes the result `sqlx-data.json`, but if necessary this can be generated using (might need to `cargo clean` first):
```
cargo sqlx prepare -- --bin zero2prod
```

To run in the production environment (the first command just checks if sqlx-data.json is populated correctly):
```
cargo sqlx prepare --check -- --bin zero2prod
sudo docker build --tag zero2prod --file Dockerfile .
sudo docker run -p 8000:8000 zero2prod
```

With the app running in either the local or production environment, you can navigate to <http://localhost:8000/login>, or simulate client requests using commands such as:
```
curl -v http://127.0.0.1:8000/health_check
curl -vd "name=le%20guin&email=ursula_le_guin%40gmail.com" http://127.0.0.1:8000/subscriptions
```

To deploy the app on DigitalOcean's servers:
```
doctl apps create --spec spec.yaml
```

To speed up the inner development loop, install a fast linker (see "LLVM stuff" above), then
```
cargo install bunyan
cargo install cargo-watch
cargo watch -x check -x test -x "run | bunyan"
```

Note: last I checked, there seems to be a problem with the production environment, where the `cargo sqlx prepare --check` and `docker run` commands both fail.
