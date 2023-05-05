#!/bin/bash
# sleep 30
mysql -f -u$MIGRATION_MYSQL_USER -p$MIGRATION_MYSQL_PASSWORD -h$MIGRATION_MYSQL_HOST -e "CREATE DATABASE IF NOT EXISTS ${MIGRATION_MYSQL_DATABASE} /*\!40100 DEFAULT CHARACTER SET utf8 */;"
mysql -u$MIGRATION_MYSQL_USER -p$MIGRATION_MYSQL_PASSWORD -h$MIGRATION_MYSQL_HOST $MIGRATION_MYSQL_DATABASE < ./migrations/create_table.sql
mysql -u$MIGRATION_MYSQL_USER -p$MIGRATION_MYSQL_PASSWORD -h$MMIGRATION_YSQL_HOST $MIGRATION_MYSQL_DATABASE < ./tmp.sql