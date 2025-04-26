.PHONY: all fetch build-generator generate lint-fix build clean print-op-ids

SRC_DIR = ./src
SPEC_DIR = ./spec
SPEC_FILE = $(SPEC_DIR)/openapi.json
SPEC_URL = "https://api.artifactsmmo.com/openapi.json"
GENERATOR_CONFIG = ./generator-config.yaml

CUSTOM_CODEGEN_JAR = artifacts-codegen/target/artifacts-codegen-openapi-generator-1.0.0.jar
OPENAPI_JAR = /usr/share/java/openapi-generator/openapi-generator-cli.jar
OPENAPI_CLASS = org.openapitools.codegen.OpenAPIGenerator

GENERATOR_CMD = java -cp $(CUSTOM_CODEGEN_JAR):$(OPENAPI_JAR) $(OPENAPI_CLASS)

all: fetch build-generator generate lint-fix build

fetch:
	curl -L $(SPEC_URL) -o "$(SPEC_FILE)" --create-dirs

build-generator:
	mvn -f artifacts-codegen package

generate: $(SPEC_FILE)
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
	mvn -f artifacts-codegen clean

print-op-ids:
	deno run --allow-read scripts/print_op_ids.ts
