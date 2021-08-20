.PHONY: build deploy

build: ## Build static binary and put it in the functions directory.
	@cargo build --release --target x86_64-unknown-linux-musl
	@mkdir -p functions
	@cp target/x86_64-unknown-linux-musl/release/hello-rs functions

deploy: build ## Deploy the site using Netlify's CLI
	@netlify deploy --prod