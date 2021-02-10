To start up the PostgreSQL database in a detached Docker container:
```
./scripts/init_db.sh
```

Once the database is running, it's possible to run and test in the local environment with `cargo run` and `cargo test`, respectively.

Once the database is running, it's also possible to statically prepare the binary's SQL queries. This repo already includes the result `sqlx-data.json`, but if necessary this can be generated using:
```
cargo clean
cargo sqlx prepare -- --bin zero2prod
```

To run in the production environment (the first command just checks if sqlx-data.json is populated correctly):
```
cargo sqlx prepare --check -- --bin zero2prod
sudo docker build --tag zero2prod --file Dockerfile .
sudo docker run -p 8000:8000 zero2prod
```

With the app running in either the local or production environment, it's easy to simulate client requests:
```
curl -v http://127.0.0.1:8000/health_check
curl -vd "name=le%20guin&email=ursula_le_guin%40gmail.com" http://127.0.0.1:8000/subscriptions
```

To deploy the app on DigitalOcean's servers:
```
doctl apps create --spec spec.yaml
```

Note to self: find out exactly when the production environment instructions work, and why the `cargo clean` above seems to be necessary.
