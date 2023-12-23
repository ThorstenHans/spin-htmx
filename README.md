# Spin & HTMX sample application

This is a sample application to demonstrate how to use [HTMX](https://htmx.org) with [Fermyon Spin](https://developer.fermyon.com/spin). The Spin app consists of two components:

- `app`: A static fileserver that servces the frontend (`./app`)
- `api`: A simple API implemented with Rust (`./api`)

For persistence, the app uses SQLite. The database schema is defined in `./migration.sql`.

## Running locally

To run the app locally, you must have `spin` CLI installed. See [https://developer.fermyon.com/spin/v2/install](https://developer.fermyon.com/spin/v2/install) for installation instructions.

Once you have `spin` installed, you can run the app locally by running the following command:

```bash
# Compile the sample
spin build

# Run the sample
spin up --sqlite @migration.sql
