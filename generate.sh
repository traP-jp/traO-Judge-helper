#!/usr/bin/env bash
docker run --rm \
  -v ${PWD}:/local openapitools/openapi-generator-cli generate \
  -i /local/schema.yaml \
  -g rust \
  -o /local/out/rust

sudo chown -R $USER:$USER out
mv out/rust schema
rm -rf out