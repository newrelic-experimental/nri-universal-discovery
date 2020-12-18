# AWS EC2 Agentless Discovery

Requirements:

- [Setup the AWS EC2 Integration](https://docs.newrelic.com/docs/integrations/amazon-integrations/aws-integrations-list/aws-ec2-monitoring-integration)
- [Generate API Key](https://one.newrelic.com/launcher/api-keys-ui.api-keys-launcher?)

Setting up the AWS connectivity integration will enumerate all available EC2s without installing any agents.

Use a decorator file to help apply credentials or any other needed meta. In the example we will use the provided key name tag to build our remote command.
[Decorator File](./decorator-file.md)

[Example Netstat Flex Config](../../examples/remote-netstat.yml)

### Example with NRQL

Enumerate EC2s with a NRQL query and then build the relevant `ssh` remote command using the ${discovery.\<value\>} replacement variables.

```yaml
---
discovery:
  ttl: 1m
  command:
    exec: /var/db/newrelic-infra/nri-universal-discovery # path to discovery binary
    env:
      NR_ACCOUNT_ID: "12345678"
      NR_API_KEY: "NRAK-XXXXXX" # API Key generated previously
      # Query ## if user tag is supplied you could add that to the query
      NR_QUERY: "SELECT \
        latest(ec2PublicIpAddress) as 'publicIpAddr', \
        latest(ec2PrivateDnsName) as 'privateDnsName', \
        latest(provider.ec2Tag_Name) as 'ec2Name', \
        latest(ec2KeyName) as 'keyName' \
        FROM ComputeSample FACET entityGuid LIMIT MAX"
      # NR_DECORATOR_FILE: /path/to/decorator/file.json ## optional
    match:
      ec2Name: /\S+/ # match is required and accepts regex when enclosed between forward slashes eg. /<regex>/
integrations:
  - name: nri-flex
    interval: 1m
    env:
      # path to Flex config
      CONFIG_FILE: /etc/newrelic-infra/integrations.d/universal-discovery-sub-configs/netstat.yml
      ALLOW_ENV_COMMANDS: true
      FLEX_META: ${discovery.discoveryMeta}
      # build the remote ssh command
      FLEX_CMD_PREPEND: "set +H && \
        ssh -oStrictHostKeyChecking=no \
        -i ${discovery.keyName} \
        ec2-user@${discovery.publicIpAddr}" # alternatively switch to privateDnsName if necessary
```
