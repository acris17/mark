APP_NAME := mark

release:
	cargo build --release
.PHONY: release

build:
	cargo build
.PHONY: build

clean:
	cargo clean
.PHONY: clean

install:
	bin install ./target/release/${APP_NAME}
.PHONY: install

uninstall:
	bin uninstall ${APP_NAME}
.PHONY: uninstall
