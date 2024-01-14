from enum import Enum, IntEnum
import os
from typing import Union

CARGO_RUN = "cargo run {switches}"
CARGO_TEST = "cargo test {switches}"


ENABLE_LOGGING = True


class Error:
    def __init__(self, message: str):
        self.message = message

    message: str


def build_and_run_project(build_for_release: bool = True) -> None:
    os.chdir("./backend")
    os.system(CARGO_RUN.format(switches="--release" if build_for_release else ""))
    os.chdir("..")


def build_and_run_int_tests() -> None:
    os.chdir("./backend")
    switches = "--test '*' "
    if ENABLE_LOGGING:
        switches += "-- --nocapture"
    os.system(CARGO_TEST.format(switches=switches))
    os.chdir("..")
