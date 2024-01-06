from scripts.helpers import (
    DbStatus,
    build_and_run_project,
    start_database,
    try_get_db_status,
)

default_folder = "./postgresql-databases/default-database"

# ---
# --- Check if postgres database is already running
# ---
db_status = try_get_db_status(default_folder)

# ---
# --- Start database if not up
# ---

if db_status == DbStatus.Down or db_status == DbStatus.Nonexistent:
    start_database(default_folder)

# ---
# --- Build and run main project
# ---
build_and_run_project(False)
