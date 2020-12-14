# Configuration

The universal discovery configuration works similarly to [container discovery](https://docs.newrelic.com/docs/integrations/host-integrations/installation/container-auto-discovery-host-integrations).

### Configuration File

- View the [usage](#Usage) options and apply into the configuration file similarly to below.
- Multiple configuration files can be created and placed into the `/etc/newrelic-infra/integrations.d/` directory.

```yaml
# example filepath/name: /etc/newrelic-infra/integrations.d/my-integration-with-discovery.yml
---
discovery:
  ttl: 1m
    # Time-To-Live of the cached discovery results, used to minimize the number of discovery processes. Define as a number followed by a time unit (s, m or h).
    # Examples: 30s, 10m, 1h, 0
    # Default: 1m
  command:
    exec: /var/db/newrelic-infra/nri-universal-discovery # path to universal discovery binary
    env:
      NR_ACCOUNT_ID: "12345678"
      NR_API_KEY: "NRAK-XXXXXX"
      ...other options
    match:
      someVariable: /\S+/ # a match is required by the Infrastructure Agent
integrations:
  - name: nri-flex
  ...
  --- truncated ---
```

### Usage

```
USAGE:
    nri-universal-discovery [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a <account-id>            New Relic Account ID [env: NR_ACCOUNT_ID=]
    -k <api-key>               New Relic One API Key [env: NR_API_KEY=]
    -d <decorator-file>        Path to decorator file [env: NR_DECORATOR_FILE=]
    -f <discovery-file>        Path to a manual discovery file [env: NR_DISCOVERY_FILE=]
    -b <meta-blacklist>        Comma separated blacklist of variables to exclude from meta (cannot
                               be used with whitelist)) [env: NR_META_BLACKLIST=]
    -w <meta-whitelist>        Comma separated whitelist of variables to include in meta (cannot be
                               used with blacklist) [env: NR_META_WHITELIST=]
    -m <mode>                  Query mode NRQL or Entity for Entity Search [env: NR_MODE=]
    -u <nerdgraph-url>         New Relic Account ID [env: NR_NERDGRAPH_URL=] [default:
                               https://api.newrelic.com/graphql]
    -q <query>                 NRQL or Entity Search Query [env: NR_QUERY=]
    -v <verbose>               Verbose logging [env: VERBOSE=]
```
