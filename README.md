# Zero2Prod
A Rust project built by following along with [Zero To Production In Rust - Luca Palmieri](https://www.zero2prod.com/index.html).

Docker is required for proper installation.

## Installation

### clone the repo:
```
git clone git@github.com:calebsg225/zero2prod-rust.git
cd zero2prod-rust
```

**Depending on where and how you installed docker, you may need to prepend `docker` commands with `sudo`, or run as root**

### Build the docker image:
Replace [IMAGE_NAME] with desired image name.
```
:~$ docker build --tag [IMAGE_NAME] --file Dockerfile .
```

### Run the docker container in the background on port 8001:
Replace [CONTAINER_NAME] with desired container name.
```
:~$ docker run -d -p 8001:8000 --name [CONTAINER_NAME] [IMAGE_NAME]
```

### You should see the new container when running `docker ps`:
```
:~$ docker ps

CONTAINER ID   IMAGE          COMMAND         CREATED        STATUS        PORTS                                         NAMES
45f5f4c516e6   [IMAGE_NAME]   "./zero2prod"   1 minute ago   Up 1 minute   0.0.0.0:8000->8000/tcp, [::]:8000->8000/tcp   [CONTAINER_NAME]

```

### Check the health of the application
A 200 status means the application is running properly
```
:~$ curl http://localhost:8001/health_check -v

* Host localhost:8001 was resolved.
* IPv6: ::1
* IPv4: 127.0.0.1
*   Trying [::1]:8001...
* Connected to localhost (::1) port 8001
> GET /health_check HTTP/1.1
> Host: localhost:8001
> User-Agent: curl/8.5.0
> Accept: */*
> 
< HTTP/1.1 200 OK
< content-length: 0
< date: Mon, 21 Apr 2025 17:39:05 GMT
< 
* Connection #0 to host localhost left intact
```
