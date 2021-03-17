#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail

test_rust() {
  rustup component add rustfmt
  cargo fmt --all -- --check

  rustup component add clippy
  cargo clippy --all-features --all-targets -- --deny warnings

  cargo check
  cargo test
}

main() {
  local -r script_folder="$(dirname "$(readlink --canonicalize "$0")")"
  local -r project_folder="$(dirname "${script_folder}")"
  cd "${project_folder}"

  docker run --entrypoint sh --rm --volume "$(pwd)":/workdir \
    evolutics/travel-kit:0.6.0 -c \
    'git ls-files -z | xargs -0 travel-kit check --'

  test_rust
}

main "$@"
