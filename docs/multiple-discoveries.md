# Multiple Discovery Configurations

### Scenarios

[1. Running multiple integrations against the same discoveries](#Running-multiple-integrations-against-the-same-discoveries)

[2. Running multiple discoveries that are different](#Running-multiple-discoveries-that-are-different)

---

## Running multiple integrations against the same discoveries

If you would like to run another integration with the same discovery settings, add another block under the integrations section.

Example path: `/etc/newrelic-infra/integrations.d/my-remote-config.yml`

Example config:

```yaml
---
discovery:
  ttl: 1m
  command:
    exec: /var/db/newrelic-infra/nri-universal-discovery
    env:
      NR_ACCOUNT_ID: "12345678"
    --------------- truncated -------------------
integrations:
  - name: nri-flex
    interval: 1m
    env:
      CONFIG_FILE: /etc/newrelic-infra/integrations.d/configs/netstat.yml
    --------------- truncated -------------------
  - name: nri-flex
    interval: 1m
    env:
      CONFIG_FILE: /etc/newrelic-infra/integrations.d/configs/another.yml
    --------------- truncated -------------------
```

## Running multiple discoveries that are different

If you would like to define another discovery based integration with a different account or query for example, simply create another configuration file.

Your New Relic Infrastructure integration configuration files are stored in the following directory:

```
/etc/newrelic-infra/integrations.d/
```

Define multiple configuration files with different names.

```
/etc/newrelic-infra/integrations.d/config-1.yml
/etc/newrelic-infra/integrations.d/config-2.yml
/etc/newrelic-infra/integrations.d/config-3.yml
```

Example configuration(s):

```yaml
---
discovery:
  ttl: 1m
  command:
    exec: /var/db/newrelic-infra/nri-universal-discovery
     # these variables could be changed per config as required
    env:
      NR_ACCOUNT_ID: "12345678"
      NR_QUERY: "something different for each config"
    --------------- truncated -------------------
integrations:
  - name: nri-flex
    interval: 1m
    env:
      CONFIG_FILE: /etc/newrelic-infra/integrations.d/configs/netstat.yml # we could use the same or different Flex config
    --------------- truncated -------------------
```
