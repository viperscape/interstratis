source $HOME/.cargo/env
pkill -f interstratis
git stash  #in case cargo.lock is different
git pull   #assumes keys are setup!
cargo update
cargo build
