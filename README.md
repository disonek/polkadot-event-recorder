# bright_inventions_task

To build and run app command should be enought 
```bash
cargo build && cargo run  
```

Wokraroud to lack of polkadot_metadata.scale -> I just copied it form subxt(Im aware its not a proper way)

The main funciton is written on basis of subxt/examples/examples/subscribe_all_events.rs

As Im docker noob I did copy form docker-compose.yml
https://github.com/substrate-developer-hub/substrate-node-template/blob/main/docker-compose.yml




sudo apt install postgresql postgresql-contrib
sudo apt install libpq-dev

sudo pg_ctlcluster 12 main start

cargo install diesel_cli --no-default-features --features postgres

echo DATABASE_URL=postgres://username:password@localhost/polkadot_events > .env

diesel setup

to silent:
Creating database: polkadot_events
FATAL:  password authentication failed for user "postgres"
FATAL:  password authentication failed for user "postgres"

sudo -i -u postgres
psql
ALTER USER postgres PASSWORD 'new_password';
\q
exit


diesel migration generate create_polkadot_events
diesel migration run
diesel print-schema > src/schema.rs





disonek@DESKTOP-EQMOURM:~/dev/bright_inventions_task$ echo DATABASE_URL=postgres://username:password@localhost/polkadot_events > .env
disonek@DESKTOP-EQMOURM:~/dev/bright_inventions_task$ diesel setup
Creating migrations directory at: /home/disonek/dev/bright_inventions_task/migrations
Creating database: polkadot_events
FATAL:  password authentication failed for user "username"
FATAL:  password authentication failed for user "username"

disonek@DESKTOP-EQMOURM:~/dev/bright_inventions_task$ echo DATABASE_URL=postgres://postgres:postgres@localhost/polkadot_events > .env
disonek@DESKTOP-EQMOURM:~/dev/bright_inventions_task$ diesel setup
Creating database: polkadot_events
FATAL:  password authentication failed for user "postgres"
FATAL:  password authentication failed for user "postgres"

disonek@DESKTOP-EQMOURM:~/dev/bright_inventions_task$ sudo diesel setup
[sudo] password for disonek: 
sudo: diesel: command not found
disonek@DESKTOP-EQMOURM:~/dev/bright_inventions_task$ diesel setup
Creating database: polkadot_events
FATAL:  password authentication failed for user "postgres"
FATAL:  password authentication failed for user "postgres"

disonek@DESKTOP-EQMOURM:~/dev/bright_inventions_task$ sudo -i -u postgres
[sudo] password for disonek: 
postgres@DESKTOP-EQMOURM:~$ psql
psql (12.12 (Ubuntu 12.12-0ubuntu0.20.04.1))
Type "help" for help.

postgres=# ALTER USER postgres PASSWORD 'postgres';
ALTER ROLE
postgres=# \q
postgres@DESKTOP-EQMOURM:~$ exit
logout
disonek@DESKTOP-EQMOURM:~/dev/bright_inventions_task$ diesel setup
Creating database: polkadot_events
disonek@DESKTOP-EQMOURM:~/dev/bright_inventions_task$ diesel migration generate create_polkadot_events
Creating migrations/2022-08-28-141406_create_polkadot_events/up.sql
Creating migrations/2022-08-28-141406_create_polkadot_events/down.sql
disonek@DESKTOP-EQMOURM:~/dev/bright_inventions_task$ diesel migration run
Running migration 2022-08-28-141406_create_polkadot_events
Executing migration script /home/disonek/dev/bright_inventions_task/migrations/2022-08-28-141406_create_polkadot_events/up.sql
Failed with: syntax error at or near "PRIAMRY"
disonek@DESKTOP-EQMOURM:~/dev/bright_inventions_task$ diesel migration run
Running migration 2022-08-28-141406_create_polkadot_events
disonek@DESKTOP-EQMOURM:~/dev/bright_inventions_task$ 