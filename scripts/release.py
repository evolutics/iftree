#!/usr/bin/env python3

import argparse
import os
import pathlib
import subprocess
import sys


def main():
    os.chdir(pathlib.Path(os.path.realpath(__file__)).parent.parent)

    parser = argparse.ArgumentParser()
    parser.add_argument("version")
    parser.add_argument("--is-real", action="store_true", help="Not a dry run.")
    arguments = parser.parse_args()

    commands = _plan_commands(arguments.version)

    print("Commands that would be run:")
    for command in commands:
        print(command)

    if arguments.is_real:
        _check_with_user("Run above comands for real?")
        for command in commands:
            subprocess.run(command, check=True)


def _plan_commands(version):
    _check_with_user(f"Commit to be released is {_get_head_commit()}, right?")
    _check_with_user(
        "Has automatic test passed for it "
        "(https://github.com/evolutics/iftree/actions/workflows/test.yml)?"
    )
    _check_with_user(
        "Have manually triggered cross-platform tests passed "
        "(https://github.com/evolutics/iftree/actions/workflows/test_other_platform.yml)?"
    )
    _check_with_user("Is changelog ready?")
    subprocess.run(["cargo", "publish", "--dry-run"], check=True)

    return [
        ["git", "tag", "--annotate", version, "--message", version],
        ["git", "push", "origin", version],
        ["cargo", "publish"],
    ]


def _check_with_user(check):
    if input(check):
        sys.exit("Aborting!")


def _get_head_commit():
    return subprocess.run(
        ["git", "rev-parse", "HEAD"], capture_output=True, check=True, text=True
    ).stdout.rstrip()


if __name__ == "__main__":
    main()
