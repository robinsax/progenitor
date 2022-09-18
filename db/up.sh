#!/bin/bash

echo "UP"

cat db/up.sql | sed -z 's/\n/ /g' | surreal sql -c http://localhost:8000 -u admin -p admin --ns testns --db testdb
