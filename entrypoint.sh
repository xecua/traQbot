export ROCKET_DATABASES="{mysql={url=\"mysql://$MARIADB_USER:$MARIADB_PASSWORD@$MARIADB_HOSTNAME/$MARIADB_DATABASE\"}}"

diesel migration run

cargo run
