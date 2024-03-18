# Important commands I learned

# Postgres + REST Api project

### docker

```bash
docker build -t <image-name> <Dir-for-dockerfile>
dokcer run [OPTIONS] --name <cont-name> <image-name>

# interactive shell
docker exec -it <container-> bash

# migrating my local db on my linux laptoip to my desktop
pg_dump ...
pg_restore -U <username> -d <db_name> --format=t ## for tar

# one you have the tar file, and your postgres container running, preferrably with a volume
cat <tar-file> | docker exec -i <container-name> pg_restore -u <username> -d <db_name> --format=t
```

### docker compose

```bash
# up down etc...

# down while destorying volumes
docker-compose 2024-03-17T09:27:34wn --volume

# migrating a pg database
docker-compose exec -T <service> pg_restore -U <username> -d <db-name> --format=t < path/to/tarball
```

### minikube

```bash
# starting
minikube start

# stopping
minikube start

# load a local image
# usefull when ipdating images
minikube image load <container-name>

# something something images?
bash (minikube docker-env)
```

### kubernetes

- #### port forwarding:

  with `fish`, running a background task can be done like so:<br />

  ```bash
  fish -c "<command>" &
  ```

  Combining it with kubectl (or `k` for short) you get something like this: <br />

  ```bash
  fish -c "k port-forward <pod> <your-port>:<pod-port>" &
  ```

- #### other stuff
  ```bash
    k apply -f <yaml-file>
  ```
  ```bash
    # configmap from file
    k create configmap <name> --from-env-file .env

    # or from ad-hoc definitions
    k create configmap <name> --from-literal=KEY=VALUE

    # same with secrets
    k create secret generic <name> --from-file=<path>

    k get <resource-name>

    # explanations
    k explain <command>

    # exec
    k exec myfirstpod -- bash

    # deleting a pod
    # keep in mind that k8s might spin up replicas as you described in your deployment.yaml
    k delete pod <deployment>

    # to delete something for good
    k scale <deployment> --replicas=0

    # disabling a cronjob
    k patch cronjobs <job-name> -p '{"spec" : {"suspend" : true }}'
  ```
