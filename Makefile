.DEFAULT_GOAL:= help

.PHONY:dependencies 
dependencies: ## download and install dependencies
	cargo update

.PHONY:test 
test: ## executes tests
	cargo test

.PHONY:build
build: ## build release
	cargo build

.PHONY:release
release: ## build release
	cargo build --release

.PHONY: install
install: release ## builds and installs workflow in alfred
	cp target/release/worktimers ~/bin/worktimers

.PHONY: help	
help: ## shows help message
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m\033[0m\n"} /^[$$()% a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)