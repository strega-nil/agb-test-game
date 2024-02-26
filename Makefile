DEBUG ?= 0

ifeq ($(DEBUG),0)
DEFAULT_GAME := test-game.gba
else
DEFAULT_GAME := test-game-dbg.gba
endif

NAME := test_game_rs

all: $(DEFAULT_GAME)

.PHONY: all run clean

TARGET_DIR := target/thumbv4t-none-eabi

RUST_SOURCES := $(wildcard src/*.rs gfx/*.aseprite)

test-game.gba: $(TARGET_DIR)/release/$(NAME)
	agb-gbafix $< -o $@

test-game-dbg.gba: $(TARGET_DIR)/debug/$(NAME)
	agb-gbafix $< -o $@

$(TARGET_DIR)/%/$(NAME): $(RUST_SOURCES) Cargo.toml
	cargo build $(if $(findstring release,$*),--release)

clean:
	rm -rf target test-game.gba test-game-dbg.gba

run: $(DEFAULT_GAME)
	open $<
