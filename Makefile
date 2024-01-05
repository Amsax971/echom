install:
	cargo build
	sudo cp ./target/debug/echom /usr/bin/

uninstall:
	sudo rm -rf /usr/bin/echom