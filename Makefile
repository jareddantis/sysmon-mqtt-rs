.PHONY: all armhf linux darwin

# Determine cross-compiler
COMPILER_DARWIN := cross
COMPILER_LINUX := cross
ifneq ($(OS),Windows_NT)
	PLATFORM = $(shell uname -s)
	ifeq ($(PLATFORM),Darwin)
		COMPILER_DARWIN := cargo
	else
		COMPILER_LINUX := cargo
	endif
endif

all:
	cargo build --release

debug:
	cargo build

armhf: # Raspberry Pi Zero, Zero W, 1
	cross build --release --target=arm-unknown-linux-gnueabihf

darwin: # macOS
	${COMPILER_DARWIN} build --release --target=x86_64-apple-darwin

linux: # Linux x86_64
	${COMPILER_LINUX} build --release --target=x86_64-unknown-linux-gnu
