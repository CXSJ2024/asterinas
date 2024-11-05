MAIN_MAKEFILE := $(firstword $(MAKEFILE_LIST))
INCLUDE_MAKEFILE := $(lastword $(MAKEFILE_LIST))
CUR_DIR := $(shell dirname $(realpath $(MAIN_MAKEFILE)))
CUR_DIR_NAME := $(shell basename $(realpath $(CUR_DIR)))
BUILD_DIR := $(CUR_DIR)/../../build
OBJ_OUTPUT_DIR := $(BUILD_DIR)/initramfs/test/$(CUR_DIR_NAME)
DEP_OUTPUT_DIR := $(BUILD_DIR)/dep/$(CUR_DIR_NAME)

ORIGINAL_CARGO_TARGET_DIR := $(CARGO_TARGET_DIR)
CARGO_TARGET_DIR := $(CUR_DIR)/target
export CARGO_TARGET_DIR

OBJ_RELEASE_PATH := $(CARGO_TARGET_DIR)/release/$(CUR_DIR_NAME)
DEP_RELEASE_PATH := $(CARGO_TARGET_DIR)/release/$(CUR_DIR_NAME).d

.PHONY: all
all:
	@mkdir -p $(OBJ_OUTPUT_DIR) $(DEP_OUTPUT_DIR)
	@cargo build --release
	@cp $(OBJ_RELEASE_PATH) $(OBJ_OUTPUT_DIR)/$(CUR_DIR_NAME)
	@cp $(DEP_RELEASE_PATH) $(DEP_OUTPUT_DIR)/$(CUR_DIR_NAME).d
	@echo "CARGO <= $(OBJ_OUTPUT_DIR)"

.PHONY: clean
clean:
	@cargo clean