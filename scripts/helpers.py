from enum import Enum, IntEnum
import os
from typing import Union

INIT_DB_STRING = "pg_ctl -D {folder} init"
START_DB_STRING = "pg_ctl -D {folder} start"
STOP_DB_STRING = "pg_ctl -D {folder} stop"
STATUS_DB_STRING = "pg_ctl -D {folder} status"
CARGO_RUN = "cargo run {switches}"
CARGO_TEST = "cargo test"


class Error:
    def __init__(self, message: str):
        self.message = message

    message: str


class DbStatus(IntEnum):
    Running = 0
    Down = 768
    Nonexistent = 1024


def try_get_db_status(folder: str) -> DbStatus:
    """
    Raises an exception if an unknown db status is returned.
    """
    db_status = os.system(STATUS_DB_STRING.format(folder=folder))
    return DbStatus(db_status)


def start_database(folder: str) -> None:
    os.system(INIT_DB_STRING.format(folder=folder))
    os.system(START_DB_STRING.format(folder=folder))


def build_and_run_project(build_for_release: bool = True) -> None:
    os.chdir("./backend")
    os.system(CARGO_RUN.format(switches="--release" if build_for_release else ""))
    os.chdir("..")


def build_and_run_int_tests() -> None:
    os.chdir("./backend")
    os.system(CARGO_TEST)
    os.chdir("..")


def stop_database(folder: str) -> None:
    os.system(STOP_DB_STRING.format(folder=folder))
