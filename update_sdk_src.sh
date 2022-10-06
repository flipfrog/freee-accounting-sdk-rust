#/bin/bash
openapi-generator generate -i https://raw.githubusercontent.com/freee/freee-api-schema/master/v2020_06_15/open-api-3/api-schema.json -g rust -o /tmp/api-rust
cp -rp /tmp/api-rust/src sdk/

