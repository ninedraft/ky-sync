My favorite e-book reader app for iOS is [KyBook 3](http://kybook-reader.com/index.html). It includes a basic HTTP content server, which allows to download and upload files to mobile device using only local network.

## Install

### From sources
```sh
git clone https://github.com/ninedraft/ky-sync.git
cargo install --path ./ky-sync
```

### From crates.io
```
cargo install ky-sync
```


## Configuration
### Env

| name      | description                     | example                  |
| --------- | ------------------------------- | ------------------------ |
| KY_SERVER | content server addr with scheme | http://192.168.1.62:8080 |
| KY_USER   | username                        | guest                    |
| KY_PASSW  | secret password                 | 12345                    |


## Usage

1. `ky-sync ./local-dir` -- will download files from KyBook device to `local-dir`
2. `ky-sync` -- will create dir `books` and download files here
