.PHONY: build deploy

build: ## Build static binary and put it in the functions directory.
	@cargo build --release --target x86_64-unknown-linux-musl
	@cp target/x86_64-unknown-linux-musl/release/hello-rs functions/hello-rs

deploy: build ## Deploy the site using Netlify's CLI
	@netlify deploy --prod