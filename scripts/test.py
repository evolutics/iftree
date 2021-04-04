#!/usr/bin/env python3

import os
import pathlib
import subprocess


def main():
    os.chdir(pathlib.Path(os.path.realpath(__file__)).parent.parent)

    _check_general_cleanliness()
    _test_rust()


def _check_general_cleanliness():
    working_folder = pathlib.Path.cwd()
    subprocess.run(
        [
            "docker",
            "run",
            "--entrypoint",
            "sh",
            "--rm",
            "--volume",
            f"{working_folder}:/workdir",
            "evolutics/travel-kit:0.6.0",
            "-c",
            "git ls-files -z | xargs -0 travel-kit check --",
        ],
        check=True,
    )


def _test_rust():
    subprocess.run(["rustup", "component", "add", "rustfmt"], check=True)
    subprocess.run(["cargo", "fmt", "--all", "--", "--check"], check=True)

    subprocess.run(["rustup", "component", "add", "clippy"], check=True)
    subprocess.run(
        [
            "cargo",
            "clippy",
            "--all-features",
            "--all-targets",
            "--",
            "--deny",
            "warnings",
        ],
        check=True,
    )

    subprocess.run(["cargo", "check"], check=True)
    subprocess.run(["cargo", "test"], check=True)

    subprocess.run(["cargo", "run", "--example", "main"], check=True)


if __name__ == "__main__":
    main()
