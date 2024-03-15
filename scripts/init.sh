# for a single image
# cat db/backup.tar | docker exec -i konyvtar-db pg_restore -U petiyeti -d elte_felho_konyvek --format=t

# using docker-compose,
# docker-compose exec -T <service> pg_restore -U <username> -d <db-name> --format=t < path/to/tarball
