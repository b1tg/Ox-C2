# Ox-C2



build server and client


```sh
cargo build
# or
cargo build --release
```

run server:

```sh
./target/debug/server

```



run client:

```sh
./target/debug/client
```


Demo: select client and execute a command

```sh
b1@pc:~/Ox-C2$ ./target/debug/server 
Starting 8 workers
Starting "actix-web-service-127.0.0.1:8080" service on 127.0.0.1:8080
>> sessions
[src/server.rs:138] keys = [
    "caf5a8faaae5d69f0b9b3d46ab62339c",
    "client0",
]
>> use caf5a8faaae5d69f0b9b3d46ab62339c
[*] use session: caf5a8faaae5d69f0b9b3d46ab62339c
(caf5a8faaae5d69f0b9b3d46ab62339c) >> cmd ls src
[*] add task success
Line: cmd ls src
(caf5a8faaae5d69f0b9b3d46ab62339c) >> got execute res: ExecuteRes { status: true, data: "c2.proto\nc2.rs\nclient.rs\nserver.rs\nutils.rs\n" }

(caf5a8faaae5d69f0b9b3d46ab62339c) >> 
```

