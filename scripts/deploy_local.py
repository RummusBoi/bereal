from scripts.helpers import (
    build_and_run_project,
)
from scripts.postgres_helpers import (
    DbStatus,
    create_and_start_database,
    create_tables,
    drop_all_tables,
    start_database,
    try_get_db_status,
)

DROP_TABLES_ON_DEPLOY = True

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
    if DROP_TABLES_ON_DEPLOY:
        drop_all_tables()
        create_tables()
elif db_status == DbStatus.Nonexistent:
    create_and_start_database(default_folder)
else:
    if DROP_TABLES_ON_DEPLOY:
        drop_all_tables()
        create_tables()


# ---
# --- Build and run main project
# ---
build_and_run_project(False)
