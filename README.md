# bereal

Our own bereal app. For legal reasons, please watch this legal video disclaimer: https://www.youtube.com/watch?v=dQw4w9WgXcQ

# How to add a new environment variable:

If you need to use a new environment variable for whatever reason, here is what you need to do:
1: Add the entry to /backend/src/configuration/.env on the form KEY=VALUE
2: Add the entry to the EnvironmentVars struct in general_helpers.rs
3: Inside generate_env_vars() you must now extend the existing EnvironmentVars struct with your new env var. This is most easily done by calling
get_env_value(...) with your env var key, and then optionally mapping it to the correct type

Note on paniccing:
If loading the env var fails for whatever reason, just panic. We don't want to be doing error handling in the case where our environment vars don't work.

# How to deploy everything locally?

1: cd into the bereal folder
2: run python3 -m scripts.deploy_local
3: press ctrl+c to tear down rust deployment again. SQL server will remain running

# How to run integration tests?

1: deploy project locally as described above
2: run python3 -m scripts.run_integration_tests

Note that this will ONLY run integration tests.
