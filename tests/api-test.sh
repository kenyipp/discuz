#!/bin/bash

TEST_COLLECTIONS_LIST=( \
	user_update_profile
)

for collection in ${TEST_COLLECTIONS_LIST[@]}; do
	newman run ./postman-collections/$collection.json \
      --reporter-cli-no-assertions         \
      --reporter-cli-no-success-assertions \
      --reporter-cli-no-console
done
