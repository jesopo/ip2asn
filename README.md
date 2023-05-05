# ip2asn

## running

currently only supports IP<->ASN mappings from https://bgp.tools

```
$ cargo run -q --release ~/table.jsonl 127.0.0.1:8083
18:59:25 [INFO] Server::run; addr=127.0.0.1:8083
18:59:25 [INFO] listening on http://127.0.0.1:8083
```

## using

when a mapping is found:

```
$ curl -D- "http://127.0.0.1:8083/1.1.1.1"
HTTP/1.1 200 OK
x-elapsed: 5.009µs
content-length: 5
date: Wed, 29 Mar 2023 18:59:48 GMT

13335
```

when a mapping is not found:

```
curl -D- "http://127.0.0.1:8083/127.0.0.1"; echo
HTTP/1.1 404 Not Found
x-elapsed: 4.060µs
content-length: 0
date: Thu, 30 Mar 2023 16:22:42 GMT


```

## benchmark

```
$ wrk -t1 -c500 -d30s http://127.0.0.1:8083/ -s random_ip.lua
Running 30s test @ http://127.0.0.1:8083/
  1 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.88ms    2.42ms  34.06ms   77.18%
    Req/Sec    68.06k     6.24k   78.10k    74.41%
  2030458 requests in 30.08s, 230.30MB read
  Non-2xx or 3xx responses: 583937
Requests/sec:  67511.05
Transfer/sec:      7.66MB
```
