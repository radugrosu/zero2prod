# create app
cat spec.yaml | doctl apps create --spec -

# show apps
doctl apps list

# create app using the correct app id (using stdin since the expected command simply doesn't work)
cat spec.yaml | doctl apps update app-id-from-doctsl-apps-list --spec -

# run migration script from local machine, after having temporarily disabled Trusted Hosts
# put DATABASE_URL in a local .remote_env file
export $(cat .remote_env) && sqlx migrate run

# add new table
sqlx migrate add add_status_to_subscriptions

# run migration script
SKIP_DOCKER=true ./scripts/init_db.sh

# backfill 
sqlx migrate add make_status_not_null_in_subscriptions

# query
curl --request POST --data 'name=guinnes&email=guiness%40gmail.com' $(doctl apps list -o json | jq -r '.[] | .default_ingress')/subscriptions --verbose

# resetting database
sqlx database reset

# dropping database in case of 'being used by other users' error
sqlx database drop --force
