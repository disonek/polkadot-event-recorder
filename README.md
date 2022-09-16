

The main funciton is written on basis of: 
https://github.com/paritytech/subxt/blob/master/examples/examples/subscribe_all_events.rs (connect to polkadot chain) 
and  
https://github.com/tensor-programming/rust_api_part_3 (database chandling and api) 

Workaroud to lack of polkadot_metadata.scale -> I just copied it form subxt(Im aware its not a proper way) 

I did try to setup docker-compose but for some reason its not working  

On ubuntu(I used WSL2) these should commands should be enought (unfortunatelly I did not have time to check it on fresh ubuntu instance)  <br />
>sudo apt update <br />
>sudo apt install postgresql postgresql-contrib libpq-dev <br />
>sudo /etc/init.d/postgresql start <br />
>cargo build && cargo run <br />

Full instruction: <br />
>sudo apt install postgresql postgresql-contrib libpq-dev <br />
>sudo /etc/init.d/postgresql start <br />
>cargo install diesel_cli --no-default-features --features postgres <br />
>echo DATABASE_URL=postgres://username:password@localhost/polkadot_events > .env <br />
>diesel setup <br />

to silent: Creating database: polkadot_events FATAL: password authentication failed for user "postgres" FATAL: password authentication failed for user "postgres" <br />
>sudo -i -u postgres psql ALTER USER postgres PASSWORD 'postgres'; \q exit <br />

>diesel migration generate create_polkadot_events <br /> 
>diesel migration run  <br />
>diesel print-schema > src/schema.rs <br />


Running app logs should look more or less like this: 
```log
Running `target/debug/bright_inventions_task`
2022-08-29T03:45:59.762804Z  INFO launch: 🔧 Configured for development.    
2022-08-29T03:45:59.762860Z  INFO launch_: address: localhost    
2022-08-29T03:45:59.762896Z  INFO launch_: port: 8000    
2022-08-29T03:45:59.762929Z  INFO launch_: log: normal    
2022-08-29T03:45:59.762961Z  INFO launch_: workers: 32    
2022-08-29T03:45:59.762993Z  INFO launch_: secret key: generated    
2022-08-29T03:45:59.763025Z  INFO launch_: limits: forms = 32KiB    
2022-08-29T03:45:59.763061Z  INFO launch_: keep-alive: 5s    
2022-08-29T03:45:59.763096Z  INFO launch_: read timeout: 5s    
2022-08-29T03:45:59.763131Z  INFO launch_: write timeout: 5s    
2022-08-29T03:45:59.763166Z  INFO launch_: tls: disabled    
2022-08-29T03:45:59.763348Z  INFO rocket::rocket: 🛰  Mounting /api/v1/:    
2022-08-29T03:45:59.763427Z  INFO _: GET /api/v1/pd_events application/json (index)    
2022-08-29T03:45:59.763505Z  INFO _: POST /api/v1/pd_events application/json (new)    
2022-08-29T03:45:59.763584Z  INFO _: GET /api/v1/pd_events/<id> application/json (show)    
2022-08-29T03:45:59.763656Z  INFO _: DELETE /api/v1/pd_events/<id> (delete)    
2022-08-29T03:45:59.763727Z  INFO _: PUT /api/v1/pd_events/<id> application/json (update)    
2022-08-29T03:45:59.763809Z  INFO rocket::rocket: 🛰  Mounting /:    
2022-08-29T03:45:59.763874Z  INFO _: GET /<file..> [5] (all)    
2022-08-29T03:45:59.763930Z  INFO _: GET / (index)    
2022-08-29T03:45:59.764019Z  INFO launch: 🚀 Rocket has launched from http://localhost:8000
2022-08-29T03:45:59.906496Z  INFO jsonrpsee_client_transport::ws: Connection established to target: Target { sockaddrs: [], host: "rpc.polkadot.io", host_header: "rpc.polkadot.io:443", _mode: Tls, path_and_query: "/" }
  Dynamic event details: 0x8129299361a70232e65dd0e91936a2271cd1f792f812f764548bbf659de0865f
    0::System::ExtrinsicSuccess 
    1::ParaInclusion::CandidateIncluded 
    2::ParaInclusion::CandidateIncluded 
    3::ParaInclusion::CandidateIncluded 
    4::ParaInclusion::CandidateIncluded 
    5::ParaInclusion::CandidateIncluded 
    6::ParaInclusion::CandidateIncluded 
    7::ParaInclusion::CandidateIncluded 
    8::ParaInclusion::CandidateIncluded 
    9::ParaInclusion::CandidateBacked 
    10::ParaInclusion::CandidateBacked 
    11::ParaInclusion::CandidateBacked 
    12::ParaInclusion::CandidateBacked 
    13::ParaInclusion::CandidateBacked 

```

I did test it using postman with http://localhost:8000/api/v1/pd_events adres 
Unfortunatelly I did not have time to write unit-tests, integration-tests  and 10 million events test 
