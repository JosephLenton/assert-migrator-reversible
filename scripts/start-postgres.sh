#!/bin/sh

docker run --name assert-migrator-reversible__postgres \
    --rm \
    --env POSTGRES_USER=user \
    --env POSTGRES_DB=assert-migrator-reversible \
    --env POSTGRES_PASSWORD=password \
    -p 5432:5432 \
    --detach postgres:15.1

sleep 5

PGPASSWORD=password psql --host=localhost --port=5432 --dbname=assert-migrator-reversible --username=user << EOM
  CREATE DATABASE "assert-migrator-reversible--1";
  CREATE DATABASE "assert-migrator-reversible--2";
  CREATE DATABASE "assert-migrator-reversible--3";
EOM
