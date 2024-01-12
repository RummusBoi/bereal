from scripts.helpers import (
    build_and_run_project,
)
from scripts.postgres_helpers import (
    DbStatus,
    create_and_start_database,
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

if db_status == DbStatus.Down:
    start_database(default_folder)

if db_status == DbStatus.Nonexistent:
    create_and_start_database(default_folder)

# ---
# --- Build and run main project
# ---
build_and_run_project(False)
