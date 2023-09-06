#! /usr/bin/fish 

cargo build --release

cp ./target/release/disc ./bin/ 

git add ./bin/disc 
git commit -m "Updated binary"

echo "Built and added to source control"
