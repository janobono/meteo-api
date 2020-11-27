# meteo-db
Docker compose to run local mariadb.

## start
```
docker compose up
```

## stop
```
docker compose down
```

## import data
```
docker cp ./data.sql meteo-db_meteo-db_1:/data.sql
docker exec -it meteo-db_meteo-db_1 bash
mysql -uapp -papp app < /data.sql
```
