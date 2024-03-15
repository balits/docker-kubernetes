@echo off
docker run --name konyvtar-db -e POSTGRES_USER=petiyeti -e POSTGRES_PASSWORD=petiyeti -e POSTGRES_DB=elte_felho_konyvek -v konyvtar-db:/var/lib/postgresql/data -p 5432:5432 -d postgres