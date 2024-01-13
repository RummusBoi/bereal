from enum import Enum, IntEnum
import os

INIT_DB_STRING = "pg_ctl -D {folder} init"
START_DB_STRING = "pg_ctl -D {folder} -l logfile.txt start"
STOP_DB_STRING = "pg_ctl -D {folder} stop"
STATUS_DB_STRING = "pg_ctl -D {folder} status"
DB_NAME = "postgres"


class ColumnType(Enum):
    VarChar = "varchar(255)"
    U8Vec = "bytea"
    I32Vec = "integer[]"
    Integer = "integer"
    BigInteger = "bigint"


class DbStatus(IntEnum):
    Running = 0
    Down = 768
    Nonexistent = 1024


table_columns = [
    (
        "comments",
        [
            ("id", ColumnType.Integer),
            ("poster_id", ColumnType.Integer),
            ("timestamp", ColumnType.BigInteger),
            ("data", ColumnType.VarChar),
        ],
    ),
    (
        "images",
        [
            ("id", ColumnType.Integer),
            ("timestamp", ColumnType.BigInteger),
            ("data", ColumnType.U8Vec),
        ],
    ),
    (
        "users",
        [
            ("id", ColumnType.Integer),
            ("friends", ColumnType.I32Vec),
            ("timestamp", ColumnType.BigInteger),
        ],
    ),
    (
        "posts",
        [
            ("id", ColumnType.Integer),
            ("poster_id", ColumnType.Integer),
            ("image", ColumnType.Integer),
            ("comments", ColumnType.I32Vec),
            ("timestamp", ColumnType.BigInteger),
        ],
    ),
]


def run_sql_command(command: str) -> None:
    """
    Will run the os command 'psql {DB_NAME} -c {command}'.
    Raises an exception if the exit code i non-zero
    """
    full_command = f"psql {DB_NAME} -c {command}"
    print(f"Running command {full_command}")
    exit_code = os.system(full_command)
    print(f"Exit code: {exit_code}")
    assert exit_code == 0


def drop_table(table: str) -> None:
    run_sql_command(f"'DROP table {table};'")


def create_table(table: str, columns_types: list[(str, ColumnType)]) -> None:
    print(f"Creating table {table}")
    columns_formatted = ",\n".join(
        [
            f"{column} {c_type.value} {'NOT NULL' if i > 0 else 'PRIMARY KEY'}"
            for (i, (column, c_type)) in enumerate(columns_types)
        ]
    )
    query = f"""'CREATE TABLE IF NOT EXISTS {table} (
    {columns_formatted}
);'"""

    run_sql_command(query)


def try_get_db_status(folder: str) -> DbStatus:
    """
    Raises an exception if an unknown db status is returned.
    """
    db_status = os.system(STATUS_DB_STRING.format(folder=folder))
    return DbStatus(db_status)


def start_database(folder: str) -> None:
    print("Booting up existing database...")
    os.system(INIT_DB_STRING.format(folder=folder))
    os.system(START_DB_STRING.format(folder=folder))
    create_tables()


def stop_database(folder: str) -> None:
    print("Stopping database...")
    os.system(STOP_DB_STRING.format(folder=folder))


def drop_all_tables() -> None:
    print("Dropping all tables...")
    for table, _ in table_columns:
        drop_table(table)


def create_tables() -> None:
    print("Creating tables in new database...")
    print(f"Table information: {table_columns}")
    for table, columns_types in table_columns:
        create_table(table, columns_types)


def create_and_start_database(folder: str) -> None:
    start_database(folder)
    create_tables()
