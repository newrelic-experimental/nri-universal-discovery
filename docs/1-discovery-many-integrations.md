# How to use a single discovery for multiple integrations

Simply add additional integration blocks as below.

Configuration location: `/etc/newrelic-infra/integrations.d/<config name>.yml`

```yaml
---
discovery:
  ttl: 1m
  command:
    exec: /var/db/newrelic-infra/nri-universal-discovery # path to discovery binary
    env:
      NR_DISCOVERY_FILE: "/root/discovery_ssh_test.json"
      NR_META_WHITELIST: "name,team"
    match:
      name: /\S+/
integrations:
 - name: nri-flex ### BLOCK 1 - First Integration
   env:
     CONFIG_FILE: /etc/newrelic-infra/integrations.d/universal-discovery-sub-configs/netstat-linux.yml
     FLEX_META: ${discovery.discoveryMeta}
     ALLOW_ENV_COMMANDS: true
     FLEX_CMD_WRAP: true
     FLEX_CMD_PREPEND: "set +H && \
       ssh -oStrictHostKeyChecking=no \
       -i ${discovery.key} \
       ${discovery.user}@${discovery.host} "
  - name: nri-flex # BLOCK 2 - Second Integration
    env:
      CONFIG_FILE: /etc/newrelic-infra/integrations.d/universal-discovery-sub-configs/linux-packages.yml
      FLEX_META: ${discovery.discoveryMeta}
      ALLOW_ENV_COMMANDS: true
      FLEX_CMD_WRAP: true
      FLEX_CMD_PREPEND: "set +H && \
        ssh -oStrictHostKeyChecking=no \
        -i ${discovery.key} \
        ${discovery.user}@${discovery.host} "
```
