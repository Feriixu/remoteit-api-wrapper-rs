# Open a recipe chooser
default:
    just --choose

# List all available recipes
list:
    just --list -u

alias t := test
# Run test using nextest
test: nextest doctest

alias tn := nextest
# Run all tests with nextest
nextest:
    cargo nextest run --all-features

alias td := doctest
# Run doctests
doctest:
    cargo test --doc --all-features

alias wt := watch-test
# Rerun test recipe on changes
watch-test:
    cargo watch -s 'just test'

alias wtn := watch-nextest
# Rerun nextest recipe on changes
watch-nextest:
    cargo watch -s 'just nextest'

alias wtd := watch-doctest
# Rerun doctest recipe on changes
watch-doctest:
    cargo watch -s 'just doctest'