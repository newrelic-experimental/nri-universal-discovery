# Applying Metadata

Universal Discovery can generate a flat json object of key pair values that your integration can consume.

This metadata is available with the following replacement value ${discovery.discoveryMeta} .

You may choose to either set a meta whitelist that will only include the variables that you select. Or alternatively a blacklist to do the inverse including everything by default and ignoring the ones defined.

The list is defined by setting comma separated attributes, and then by using the corresponding environment variable or the cli flag to set. See [configuration](/docs/configuration.md) for detail.

The below example uses a whitelist and only includes several attributes.

Configuration location: `/etc/newrelic-infra/integrations.d/<config name>.yml`

```yaml
---
discovery:
  ttl: 1m
  command:
    exec: /var/db/newrelic-infra/nri-universal-discovery
    env:
      NR_ACCOUNT_ID: "12345678"
      NR_API_KEY: "NRAK-XXXXXX"
      NR_QUERY: "SELECT \
        latest(datacenterName) as 'dcName', \
        latest(vmConfigName) as 'vmName', \
        latest(vmHostname) as 'vmHostname', \
        latest(entityGuid) as 'remoteEntityGuid', \
        latest(entityName) as 'remoteEntityName' \
        FROM VSphereVmSample FACET entityName,entityGuid LIMIT MAX"
      NR_DECORATOR_FILE: /path/to/decorator/file.json
      NR_META_WHITELIST: "remoteEntityName,remoteEntityGuid,vmName,dcName,vmHostname" # <----------
    match:
      vmName: /\S+/
integrations:
  - name: nri-flex
    interval: 1m
    env:
      # path to Flex config
      CONFIG_FILE: /etc/newrelic-infra/integrations.d/universal-discovery-sub-configs/netstat.yml
      ALLOW_ENV_COMMANDS: true
      FLEX_META: ${discovery.discoveryMeta} # <------
      -------------- truncated --------------
```
