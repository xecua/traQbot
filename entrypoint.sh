export ROCKET_DATABASES="{mysql={url=\"mysql://$MARIADB_USER:$MARIADB_PASSWORD@$MARIADB_HOSTNAME/$MARIADB_DATABASE\"}}"
export DATABASE_URL="mysql://$MARIADB_USER:$MARIADB_PASSWORD@$MARIADB_HOSTNAME/$MARIADB_DATABASE"
export RUST_LOG=debug

dockerize -timeout 60s -wait tcp://mysql:3306

diesel migration run

RUST_BACKTRACE=1 cargo run
