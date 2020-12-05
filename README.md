# nri-universal-discovery

Dynamically create discovery options from the following sources:

- NRQL
- Entities
- Files

## Usage

```
USAGE:
    nri-universal-discovery [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a <account-id>            New Relic Account ID [env: NR_ACCOUNT_ID=]
    -k <api-key>               New Relic One API Key [env: NR_API_KEY=]
    -d <decorator-file>        Decorate [env: NR_DECORATOR_FILE=]
    -f <discovery-file>        Serve a custom discovery file [env: NR_DISCOVERY_FILE=]
    -m <mode>                  Query mode NRQL or NerdGraph [env: NR_MODE=]
    -u <nerdgraph-url>         New Relic Account ID [env: NR_NERDGRAPH_URL=] [default:
                               https://api.newrelic.com/graphql]
    -q <query>                 Query [env: NR_QUERY=]
    -v <verbose>               Verbose logging [env: VERBOSE=]
```
