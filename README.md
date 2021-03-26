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
Document Path:          /httptest/wikipedia_russia.html
Document Length:        954824 bytes

Concurrency Level:      10
Time taken for tests:   7.173 seconds
Complete requests:      20000
Failed requests:        0
Total transferred:      19101420000 bytes
HTML transferred:       19096480000 bytes
Requests per second:    2788.05 [#/sec] (mean)
Time per request:       3.587 [ms] (mean)
Time per request:       0.359 [ms] (mean, across all concurrent requests)
Transfer rate:          2600381.19 [Kbytes/sec] received

```
### rust web-server

```
Document Path:          /httptest/wikipedia_russia.html
Document Length:        954824 bytes

Concurrency Level:      10
Time taken for tests:   9.903 seconds
Complete requests:      20000
Failed requests:        0
Total transferred:      19099500000 bytes
HTML transferred:       19096480000 bytes
Requests per second:    2019.60 [#/sec] (mean)
Time per request:       4.951 [ms] (mean)
Time per request:       0.495 [ms] (mean, across all concurrent requests)
Transfer rate:          1883462.86 [Kbytes/sec] received

```





