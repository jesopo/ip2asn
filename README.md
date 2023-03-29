# ip2asn

```
$ cargo run -q --release ~/table.jsonl 127.0.0.1:8083
18:59:25 [INFO] Server::run; addr=127.0.0.1:8083
18:59:25 [INFO] listening on http://127.0.0.1:8083
```

```
$ curl -D- "http://127.0.0.1:8083/1.1.1.1"
HTTP/1.1 200 OK
x-elapsed: 5.009µs
content-length: 5
date: Wed, 29 Mar 2023 18:59:48 GMT

13335
```
