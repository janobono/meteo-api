# meteo-api
Simple meteo-api for Tomas

- [docker mariadb](./meteo-db/README.md)

## build
- system dependencies
```

```

- build
```
cargo build --release
```

- run
```
cargo run
```

- test
```
curl -s -X GET 'http://localhost:8080/api/data?timestampFrom=1606374000&timestampTo=1606460400&columns=outTemp,dewpoint'
```
