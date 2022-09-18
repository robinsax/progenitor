#!/bin/bash

echo "DOWN"

cat db/down.sql | sed -z 's/\n/ /g' | surreal sql -c http://localhost:8000 -u admin -p admin --ns testns --db testdb
