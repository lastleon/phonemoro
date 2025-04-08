#!/usr/bin/bash
set -e

# prepare
echo "Download us_gold.json, us_silver.json, and the LICENSE from Misaki repository (https://github.com/hexgrad/misaki/)"

target_dir=data
mkdir -p $target_dir

# download
for filename in us_gold.json us_silver.json
do
    curl https://raw.githubusercontent.com/hexgrad/misaki/refs/heads/main/misaki/data/$filename -o $target_dir/$filename
done

curl https://raw.githubusercontent.com/hexgrad/misaki/refs/heads/main/LICENSE -o $target_dir/LICENSE
