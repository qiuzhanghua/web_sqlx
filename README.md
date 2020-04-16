# web_sqlx

## for mysql
```bash
cargo run
# or
DATABASE_URL="mysql://app:app@localhost:3306/app" cargo run --no-default-features --features with-mysql
```

## for pg
```bash
DATABASE_URL="postgres://app:app@localhost:5432/app" cargo run --no-default-features --features with-postgres
```

## Performance

MacBook Pro (Retina, Mid 2012)

CPU 2.6GHz, 4 core, Intel Core i7

Memory 16GB 1600 MHz DDR3

macOS Catalina

about 17k Requests/sec, query from MySQL 8.0.19 to Browser.

```text
➜  ~ wrk -t10 -c20 -d30s http://127.0.0.1:8080
Running 30s test @ http://127.0.0.1:8080
  10 threads and 20 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.97ms    7.01ms 126.92ms   98.10%
    Req/Sec     1.68k   383.43     3.37k    74.50%
  501264 requests in 30.01s, 73.14MB read
Requests/sec:  16704.91
Transfer/sec:      2.44MB
➜  ~ wrk -t10 -c40 -d30s http://127.0.0.1:8080
Running 30s test @ http://127.0.0.1:8080
  10 threads and 40 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.78ms    6.28ms 127.54ms   98.82%
    Req/Sec     1.78k   159.41     2.41k    78.70%
  530257 requests in 30.02s, 77.37MB read
Requests/sec:  17664.70
Transfer/sec:      2.58MB
➜  ~ wrk -t20 -c40 -d30s http://127.0.0.1:8080
Running 30s test @ http://127.0.0.1:8080
  20 threads and 40 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     3.00ms    7.40ms 127.85ms   98.75%
    Req/Sec     0.87k    96.15     1.42k    78.80%
  518427 requests in 30.03s, 75.64MB read
Requests/sec:  17265.30
Transfer/sec:      2.52MB
```
