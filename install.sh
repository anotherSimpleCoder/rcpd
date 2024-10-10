#!/bin/bash
cargo build --release

if [ ! -d "$HOME/.rcp" ]; then
	mkdir "$HOME/.rcp"
fi

config="{
	\"port\":3000,
	\"out_path\":\"$HOME/Downloads\"
}"

echo $config > $HOME/.rcp/config.json
cp ./target/release/rcpd $HOME/.rcp/rcpd
