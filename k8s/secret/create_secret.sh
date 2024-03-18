#! /bin/bash

kubectl create secret generic donttellanyone \
	--from-literal=POSTGRES_USER=petiyeti \
	--from-literal=POSTGRES_PASSWORD=petiyeti \
	--from-literal=POSTGRES_DB=elte_felho_konyvek

# apiVersion: v1
# kind: Secret
# metadata:
#   name: donttellanyone
# type: Opaque
# data:
#   POSTGRES_USER: "petiyeti"
#   POSTGRES_PASSWORD: "petiyeti"
#   POSTGRES_DB: "elte_felho_konyvek"
