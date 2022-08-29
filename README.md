# bright_inventions_task

The main funciton is written on basis of:
https://github.com/paritytech/subxt/blob/master/examples/examples/subscribe_all_events.rs (connect to polkadot chain)
and 
https://github.com/tensor-programming/rust_api_part_3 (database chandling and api)

Workaroud to lack of polkadot_metadata.scale -> I just copied it form subxt(Im aware its not a proper way)

I did try to setup docker-compose but for some reason its not working 

on ubunu these should be enought(unfortunatelly I did not have time to check it on fres ubuntu instance) 
sudo apt update
sudo apt install postgresql postgresql-contrib libpq-dev
/etc/init.d/postgresql start
cargo build && cargo run


Full instruction:
>sudo apt install postgresql postgresql-contrib libpq-dev
>/etc/init.d/postgresql start
>cargo install diesel_cli --no-default-features --features postgres
>echo DATABASE_URL=postgres://username:password@localhost/polkadot_events > .env
>diesel setup

to silent: Creating database: polkadot_events FATAL: password authentication failed for user "postgres" FATAL: password authentication failed for user "postgres"
>sudo -i -u postgres psql ALTER USER postgres PASSWORD 'new_password'; \q exit

>diesel migration generate create_polkadot_events 
>diesel migration run 
>diesel print-schema > src/schema.rs