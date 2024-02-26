DEBUG ?= 0

ifeq ($(DEBUG),0)
DEFAULT_GAME := test-game.gba
else
DEFAULT_GAME := test-game-dbg.gba
endif

all: $(DEFAULT_GAME)

.PHONY: run clean

TARGET_DIR := target/thumbv4t-none-eabi

RUST_SOURCES := $(wildcard src/**/*.rs)

test-game.gba: $(TARGET_DIR)/release/agb_template
	agb-gbafix $< -o $@

test-game-dbg.gba: $(TARGET_DIR)/debug/agb_template
	agb-gbafix $< -o $@

$(TARGET_DIR)/%/agb_template: $(RUST_SOURCES) Cargo.toml
	cargo build $(if $(findstring release,$*),--release)

clean:
	rm -rf target test-game.gba test-game-dbg.gba

run: $(DEFAULT_GAME)
	open $<
