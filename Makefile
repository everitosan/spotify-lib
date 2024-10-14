# This should be run 
test:
	@cargo test modules::oauth::utils::callback::test::test_spotify_callbacks
	@cargo test modules::oauth::utils::callback::test::test_spotify_callback
	@cargo test modules::oauth::config::test
	@cargo test modules::oauth::client::test

test-all: test
