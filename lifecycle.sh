
source $HOME/.cargo/env
pkill -f interstratis
pkill -f lifecycle
git stash
git pull
cargo update
cargo build
cp ./target/debug/lifecycle ./target/debug/lifecycle_

./target/debug/lifecycle_
