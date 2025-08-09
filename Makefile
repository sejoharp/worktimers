.DEFAULT_GOAL:= help

.PHONY:dependencies 
dependencies: ## downloads and installs dependencies
	cargo update

.PHONY:test 
test: ## executes tests
	cargo test

.PHONY:build
build: ## builds binary with debug infos
	cargo build

.PHONY:release
release: ## builds release binary
	cargo build --release

.PHONY: install
install: release ## builds and installs `worktimers` binary into ~/bin directory
	cp target/release/worktimers ~/bin/worktimers

.PHONY: help	
help: ## shows help message
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m\033[0m\n"} /^[$$()% a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

.PHONY:version-update
version-update: ## updates version in Cargo.toml
	scripts/bump-version.sh patch

.PHONY: tag-release
tag-release:
	git tag v$(shell cargo pkgid | cut -d# -f2 | cut -d: -f2)

.PHONY: push-release
push-release:
	git push
	git push --tags
