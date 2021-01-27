# SSH Agentless Remote Execution

### Requirements

- SSH client
- [Generate API Key](https://one.newrelic.com/launcher/api-keys-ui.api-keys-launcher?)

### Example with Discovery File

Use a discovery file and then build the relevant `ssh` remote command using the ${discovery.\<value\>} replacement variables.
[Discovery File Example](./discovery-file.md)

Configuration location: `/etc/newrelic-infra/integrations.d/<config name>.yml`

```yaml
---
discovery:
  ttl: 1m
  command:
    exec: /var/db/newrelic-infra/nri-universal-discovery # path to discovery binary
    env:
      NR_DISCOVERY_FILE: "/root/manual_discovery_file.json" # path to a discovery file
      NR_META_WHITELIST: "name,team"
    match:
      name: /\S+/
integrations:
  - name: nri-flex
    interval: 1m
    env:
      # path to Flex config
      CONFIG_FILE: /etc/newrelic-infra/integrations.d/universal-discovery-sub-configs/ebpf-tcplife.yml
      FLEX_META: ${discovery.discoveryMeta}
      ALLOW_ENV_COMMANDS: true
      FLEX_CMD_WRAP: true
      # build remote ssh command
      # below allows sudo capabilites over ssh
      FLEX_CMD_PREPEND: "set +H && echo ${discovery.pass} | ssh -t ${discovery.user}@${discovery.host} " # ensure space is left at the end of the command
```

[Example Netstat Flex Config](/examples/ebpf-tcplife.yml)

### Example Discovery File Contents

eg. /root/manual_discovery_file.json

```json
{
  "defaults": {
    "user": "kav",
    "pass": "myPass!"
  },
  "discovery_items": [
    {
      "name": "ubuntu-newrelic",
      "host": "192.168.0.205",
      "team": "batman"
    }
  ]
}
```
