# meteo-api
Simple meteo-api for Tomas

## assignment
Simple public rest api with one GET method to publish data from mariadb database.

- GET /api/data
  - query params
    - timestampFrom
      - epoch time in millis
    - timestampTo
      - epoch time in millis
    - columns
      - String parameter
      - possible values: inTemp,outTemp,barometer,inHumidity,outHumidity,windSpeed,windDir,windGust,windGustDir,rainRate,
      rain,dewpoint,windchill,heatindex,UV
  - output as a JSON array must always contains datetime column value.

- example
`/api/data?timestampFrom=1606374000&timestampTo=1606460400&columns=outTemp,dewpoint`

```json
[
  {"datetime":1606375200,"outTemp":-1.2,"dewpoint":-5.59277576087998},
  {"datetime":1606375500,"outTemp":-1.2,"dewpoint":-5.59277576087998},
  {"datetime":1606375800,"outTemp":-1.2,"dewpoint":-5.59277576087998}
]
```

## db
- [docker mariadb](./meteo-db/README.md)

## build
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
curl -s -X GET 'http://localhost:8080/api/data?timestampFrom=1606374000&timestampTo=1606460400&columns=inTemp,outTemp,barometer,inHumidity,outHumidity,windSpeed,windDir,windGust,windGustDir,rainRate,rain,dewpoint,windchill,heatindex,UV'
```

## env
|NAME|DEFAULT VALUE|
|---|---|
|METEO_API_PORT|8080|
|METEO_API_DB_URL|mysql://app:app@localhost:3306/app|

## system dependencies
```
sudo apt install pkg-config libssl-dev
```
