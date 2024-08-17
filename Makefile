SRC_DIR = ./src
SPEC_DIR = ./spec
SPEC_FILE = $(SPEC_DIR)/openapi.json
SPEC_URL = "https://api.artifactsmmo.com/openapi.json"

GENERATOR_CMD = openapi-generator-cli
GENERATOR_CONFIG = ./generator-config.yaml

all: fetch generate lint-fix build

fetch:
	curl -L $(SPEC_URL) -o "$(SPEC_FILE)" --create-dirs

generate:
	$(GENERATOR_CMD) generate -i "$(SPEC_FILE)" -c "$(GENERATOR_CONFIG)"

lint-fix:
	cargo clippy --fix --allow-dirty
	cargo fmt

build:
	cargo build

clean:
	cargo clean
	rm -rf $(SRC_DIR)
	rm -rf $(SPEC_DIR)

.PHONY: all fetch generate lint-fix build clean
