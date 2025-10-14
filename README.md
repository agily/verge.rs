Verge.io Rust SDK created with progenitor

1. Run the generator to preprocess the OpenAPI spec and add operation IDs required by progenitor
   `cargo run -p generator --release`
   It doesn't overwrite the target file `generator/swagger/generated-opids.json` so you may need to clean this first.
2. Run progenitor to generate the SDK
   `cargo progenitor -i generator/swagger/generated-opids.json -o sdk -n verge_rs_sdk -v 0.1.0 --interface builder --license-name "UNLICENSED"`

## CLI

This workspace includes a simple CLI to create a Tenant using the SDK.

- Build: `cargo build -p verge-cli --release`
- Run: `./target/release/verge-cli --base-url https://your-verge/api --name mytenant [options]`

Options
- `--base-url`: VergeIO API base URL (also `VERGE_BASE_URL`)
- `--name`: Tenant name
- `--description`: Optional description
- `--owner`: Optional owner identifier
- `--password`: Optional initial password
- `--help-url`: Optional help URL override
- `--isolate`: Enable tenant isolation
- `--expose-cloud-snapshots`: Ensure cloud snapshots are exposed
- `--token`: Bearer token (also `VERGE_TOKEN`)
- `--header Name:Value`: Extra header (repeatable), e.g. `--header 'Cookie:vmsession=...'`

Examples
- With cookie: `verge-cli --base-url https://verge.example/api --name demo --header 'Cookie:vmsession=abc123'`
- With bearer token: `VERGE_TOKEN=xyz verge-cli --base-url https://verge.example/api --name demo`
