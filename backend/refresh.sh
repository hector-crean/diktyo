#!/bin/bash

# give permission to script via: chmod +x ./refresh.sh
sqlx database drop
sqlx database create
sqlx migrate run

