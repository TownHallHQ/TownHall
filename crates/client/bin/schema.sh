#!/bin/sh

curl -X POST http://127.0.0.1:7878/graphql \
    -H "Expect:" \
    -H 'Content-Type: application/json; charset=utf-8' \
    -d @query.json \
    -o schema.json
