# SSH Agentless Remote Execution

### Requirements

- SSH client
- [Generate API Key](https://one.newrelic.com/launcher/api-keys-ui.api-keys-launcher?)

### Example with Discovery File

Use a discovery file and then build the relevant `ssh` remote command using the ${discovery.\<value\>} replacement variables.
[Discovery File Example](./discovery-file.md)

```yaml
---
discovery:
  ttl: 1m
  command:
    exec: /var/db/newrelic-infra/nri-universal-discovery # path to discovery binary
    env:
      NR_DISCOVERY_FILE: "/root/manual_discovery_file.json" # path to a discovery file
    match:
      name: /\S+/
integrations:
  - name: nri-flex
    interval: 1m
    env:
      # path to Flex config
      CONFIG_FILE: /etc/newrelic-infra/integrations.d/configs/remote-netstat.yml
      ALLOW_ENV_COMMANDS: true
      # build remote ssh command
      FLEX_CMD_PREPEND: "set +H && \
        ssh -oStrictHostKeyChecking=no \
        -i ${discovery.key} \
        ${discovery.user}@${discovery.host}"
```

[Example Netstat Flex Config](../../examples/remote-netstat.yml)
