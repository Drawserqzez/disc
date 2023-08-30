#! /usr/bin/fish 

cargo build --release

docker-compose up -d

./target/release/disc &

echo "Everything is deployed now :D"
