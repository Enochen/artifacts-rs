SPEC_URL = "https://api.artifactsmmo.com/openapi.json"
SPEC_DIR = ./spec
SPEC_FILE = $(SPEC_DIR)/openapi.json
GENERATOR_CONFIG = ./generator-config.yaml

all: fetch generate lint-fix

fetch:
	curl -L $(SPEC_URL) -o "$(SPEC_FILE)" --create-dirs

generate:
	openapi-generator-cli generate -i "$(SPEC_FILE)" -c "$(GENERATOR_CONFIG)"

lint-fix:
	cargo clippy --fix --allow-dirty
	cargo fmt

clean:
	cargo clean
	rm -rf $(SPEC_DIR)

.PHONY: all fetch generate lint-fix clean
