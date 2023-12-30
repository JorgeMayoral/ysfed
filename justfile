_default:
  @just --list

# Runs the cli
run:
  cargo run -- $(ARGS)
