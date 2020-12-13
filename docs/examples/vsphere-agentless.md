# vSphere Agentless Remote Execution

### Requirements

- [Install the vSphere Integration](https://docs.newrelic.com/docs/integrations/host-integrations/host-integrations-list/vmware-vsphere-monitoring-integration#)
- [Install the govc cli](https://github.com/vmware/govmomi/tree/master/govc#installation)
- [Generate API Key](https://one.newrelic.com/launcher/api-keys-ui.api-keys-launcher?)

Installing the vSphere integration will enumerate all available VMs and the `govc` cli will allow a seamless agentless remote experience into the guests.

Use a decorator file to help apply credentials or any other needed meta.
[Decorator File](./decorator-file.md)

[Example Netstat Flex Config](../../examples/remote-netstat.yml)

### Example with NRQL

Enumerate Guest VMs with a NRQL query and then build the relevant `govc` remote command using the ${discovery.\<value\>} replacement variables.

```
---
discovery:
  ttl: 1m
  command:
    exec: /var/db/newrelic-infra/nri-universal-discovery # path to discovery binary
    env:
      NR_ACCOUNT_ID: "12345678"
      NR_API_KEY: "NRAK-XXXXXX" # API Key generated previously
      # Query
      NR_QUERY: "SELECT \
        latest(datacenterName) as 'dcName', \
        latest(vmConfigName) as 'vmName', \
        latest(vmHostname) as 'vmHostname' \
        FROM VSphereVmSample FACET entityName"
      NR_DECORATOR_FILE: /path/to/decorator/file.json
    match:
      vmName: /\S+/ # match is required and accepts regex when enclosed between forward slashes eg. /<regex>/
integrations:
  - name: nri-flex
    env:
      # path to Flex config
      CONFIG_FILE: /etc/newrelic-infra/integrations.d/configs/netstat.yml
      FLEX_META: ${discovery.discoveryMeta}
      # build the remote command with govc
      REMOTE_CMD: "set +H && \
        govc guest.run -k \
        -u \"https://${discovery.dcUser}:${discovery.dcPass}@192.168.0.210\" \
        -dc \"${discovery.dcName}\" \
        -k -vm \"${discovery.vmName}\" \
        -l \"${discovery.guestUser}:${discovery.guestPass}\""
```

### Example with Entity Search

Enumerate Guest VMs with a Entity Search Query and then build the relevant `govc` remote command using the ${discovery.\<value\>} replacement variables.

```
---
discovery:
  ttl: 1m
  command:
    exec: /var/db/newrelic-infra/nri-universal-discovery # path to discovery binary
    env:
      NR_ACCOUNT_ID: "12345678"
      NR_API_KEY: "NRAK-XXXXXX" # API Key generated previously
      # Query
      NR_QUERY: "type='VSPHEREVM'"
      NR_MODE: "entity"
      NR_DECORATOR_FILE: /path/to/decorator/file.json
    match:
      name: /\S+/ # match is required and accepts regex when enclosed between forward slashes eg. /<regex>/
integrations:
  - name: nri-flex
    env:
      # path to Flex config
      CONFIG_FILE: /etc/newrelic-infra/integrations.d/configs/netstat.yml
      FLEX_META: ${discovery.discoveryMeta}
      # build the remote command with govc
      REMOTE_CMD: "set +H && \
        govc guest.run -k \
        -u \"https://${discovery.dcUser}:${discovery.dcPass}@192.168.0.210\" \
        -dc \"${discovery.dcName}\" \
        -k -vm \"${discovery.vmName}\" \
        -l \"${discovery.guestUser}:${discovery.guestPass}\""
```
