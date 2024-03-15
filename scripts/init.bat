@echo off
type db\backup.tar | docker exec -i konyvtar-db pg_restore -U petiyeti -d elte_felho_konyvek --format=t