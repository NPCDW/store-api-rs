# store-api-rs

![https://www.rust-lang.org/](https://img.shields.io/badge/rust-1.64-dea584.svg)
![https://actix.rs/](https://img.shields.io/badge/actix_web-4-purple.svg)
![https://crates.io/crates/tracing](https://img.shields.io/badge/tracing-0.1-yellow.svg)
![https://crates.io/crates/rusqlite](https://img.shields.io/badge/rusqlite-0.28-blue.svg)
![https://crates.io/crates/r2d2](https://img.shields.io/badge/r2d2-0.8-green.svg)
![https://crates.io/crates/sea-query](https://img.shields.io/badge/sea_query-0-red.svg)

`rust web` 应用模板，`web` 框架使用 `actix-web`

实现了 [store-api](https://github.com/NPCDW/store-api) 和 [store-api-vertx](https://github.com/NPCDW/store-api-vertx) 同样的功能，但不得不说， `rust` 真乃内存管理大师， `docker` 中初始内存占用只有 `2M` ，调用了几次 `api` ，内存才到 `3M`

## Docker 自行构建

首先先拉取本项目
```bash
git clone https://github.com/NPCDW/store-api-rs.git
```

创建并自行修改 `config.yml` 配置文件，配置文件模板在这 [config.example.yml](resources/config/config.example.yml)
```bash
mkdir -p config
vi config/config.yml
```

```base
docker compose up -d
```