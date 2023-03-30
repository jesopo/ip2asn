# ip2asn

## running

currently only supports IP<->ASN mappings from https://bgp.tools

```
$ cargo run -q --release ~/table.jsonl 127.0.0.1:8083
18:59:25 [INFO] Server::run; addr=127.0.0.1:8083
18:59:25 [INFO] listening on http://127.0.0.1:8083
```

## using

```
$ curl -D- "http://127.0.0.1:8083/1.1.1.1"
HTTP/1.1 200 OK
x-elapsed: 5.009µs
content-length: 5
date: Wed, 29 Mar 2023 18:59:48 GMT

13335
```

## benchmark

```
$ wrk -t1 -c500 -d30s http://127.0.0.1:8083 -s random_ip.lua
Running 30s test @ http://127.0.0.1:8083
  1 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.24ms    2.45ms  36.88ms   75.77%
    Req/Sec    60.38k     6.14k   73.08k    77.97%
  1795945 requests in 30.01s, 256.23MB read
  Non-2xx or 3xx responses: 840337
Requests/sec:  59837.97
Transfer/sec:      8.54MB
```
