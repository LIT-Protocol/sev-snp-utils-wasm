#!/bin/bash

echo "You ran build.sh first right?"

echo "Copying files to sdk..."

cp convertWasmToJs/sev-snp-utils-sdk.js ../js-sdk/packages/sev-snp-utils-sdk/src/lib/sev-snp-utils-sdk.ts

echo "Prepending lines to sev-snp-utils-sdk.ts..."


echo "import * as pako from 'pako';" | cat - ../js-sdk/packages/sev-snp-utils-sdk/src/lib/sev-snp-utils-sdk.ts > temp && mv temp ../js-sdk/packages/sev-snp-utils-sdk/src/lib/sev-snp-utils-sdk.ts



echo "// @ts-nocheck" | cat - ../js-sdk/packages/sev-snp-utils-sdk/src/lib/sev-snp-utils-sdk.ts > temp && mv temp ../js-sdk/packages/sev-snp-utils-sdk/src/lib/sev-snp-utils-sdk.ts

