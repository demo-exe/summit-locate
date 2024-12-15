all:
	cargo run

windows:
	cross build --target x86_64-pc-windows-gnu --release

# jesli wystapi blad z glibcem to trzeba cargo clean
