# Open a recipe chooser
default:
    just --choose

# List all available recipes
list:
    just --list -u

alias wn := watch-nextest
# Run all tests with the nextest runner, rerunning when files change
watch-nextest:
    cargo watch -x 'nextest run --all-features'