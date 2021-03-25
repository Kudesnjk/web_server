# Веб-сервер
Highload. Домашнее задание №2. Разработка веб-сервера для отдачи статики с диска.

## Использование:

#### Config: `/etc/httpd.conf`
```
cpu_limit 4       # maximum CPU count to use (for non-blocking servers)
thread_limit 256  # maximum simultaneous connections (for blocking servers)
document_root /var/www/html
```

```
git clone https://github.com/Kudesnjk/web_server
cd web_server
docker build -t web_server .
docker run -p 80:80 -v /etc/httpd.conf:/etc/httpd.conf:ro -v /var/www/html:/var/www/html:ro --name web_server web_server
```
## Benchmarks

### nginx

```
Running 30s test @ http://localhost/httptest/wikipedia_russia.html
  8 threads and 2000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   294.12ms   35.20ms 573.46ms   90.36%
    Req/Sec   463.39    315.42     1.79k    72.99%
  102058 requests in 30.08s, 90.78GB read
  Socket errors: connect 987, read 0, write 0, timeout 0
Requests/sec:   3392.92
Transfer/sec:      3.02GB
```
### rust web-server

```
Running 30s test @ http://localhost:8080/httptest/wikipedia_russia.html
  8 threads and 2000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   120.34ms  104.38ms   3.38s    94.27%
    Req/Sec   304.44    105.37   646.00     65.55%
  72644 requests in 30.08s, 64.61GB read
  Socket errors: connect 987, read 72630, write 0, timeout 0
Requests/sec:   2415.09
Transfer/sec:      2.15GB
```





