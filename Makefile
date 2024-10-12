# This should be run 
test:
	@cargo test modules::oauth::config::test
	@cargo test modules::oauth::client::test

test-all: test
