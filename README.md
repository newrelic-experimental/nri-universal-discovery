[![New Relic Experimental header](https://github.com/newrelic/opensource-website/raw/master/src/images/categories/Experimental.png)](https://opensource.newrelic.com/oss-category/#new-relic-experimental)

# nri-universal-discovery

Universal Discovery allows you to dynamically or manually define discovery items for Infrastructure Integrations.
This discovery mechanism is particularly useful for agentless instrumentation and data collection, view the [documentation](#documentation) for further information.

---

Create discovery options from the following sources:

- NRQL
- Entity Search
- Discovery File

---

## Installation

- Download `nri-universal-discovery` from the releases section or compile and place the binary into a directory accessible by the Infrastructure Agent eg.

```
/var/db/newrelic-infra/nri-universal-discovery
```

---

## Documentation

### System Requirements

- A single Linux VM/Host/Machine to deploy too.
- New Relic Infrastructure Agent 1.14.1 or above (bundles Flex 1.3.8).
- Flex 1.3.8 or above.

### Getting started

- [Installing Universal Discovery](#Installation)
- [Universal Discovery configuration options](/docs/configuration.md)
- [Testing returned discoveries](/docs/faq/testing-discoveries.md)
- [Examples](examples/README.md)
  - [vSphere/vCenter Agentless Remote Execution](/docs/examples/vsphere-agentless.md)
  - [SSH Agentless Remote Execution](/docs/examples/ssh-agentless.md)
  - [AWS EC2 Agentless Remote Execution](/docs/examples/aws-ec2-agentless.md)
  - [Discovery File Format](/docs/examples/discovery-file.md)
  - [Decorator File Format](/docs/examples/decorator-file.md)
- [Optimizing discovery performance](/docs/faq/optimize.md)
- [Securing credentials]()
- [Configure multiple discovery integrations](/docs/faq/multiple-discoveries.md)

---

## Building

### Setup

```
make setup
```

### Linux

```
make build-linux
```

### Windows

```
make build-windows
```

---

## Testing

```
cargo test
or
make test
```

---

## Support

New Relic has open-sourced this project. This project is provided AS-IS WITHOUT WARRANTY OR DEDICATED SUPPORT. Issues and contributions should be reported to the project here on GitHub. We encourage you to bring your experiences and questions to the [Explorers Hub](https://discuss.newrelic.com) where our community members collaborate on solutions and new ideas.

## Contributing

We encourage your contributions to improve nri-universal-discovery! Keep in mind when you submit your pull request, you'll need to sign the CLA via the click-through using CLA-Assistant. You only have to sign the CLA one time per project. If you have any questions, or to execute our corporate CLA, required if your contribution is on behalf of a company, please drop us an email at opensource@newrelic.com.

**A note about vulnerabilities**

As noted in our [security policy](../../security/policy), New Relic is committed to the privacy and security of our customers and their data. We believe that providing coordinated disclosure by security researchers and engaging with the security community are important means to achieve our security goals.

If you believe you have found a security vulnerability in this project or any of New Relic's products or websites, we welcome and greatly appreciate you reporting it to New Relic through [HackerOne](https://hackerone.com/newrelic).

## License

nri-universal-discovery is licensed under the [Apache 2.0](http://apache.org/licenses/LICENSE-2.0.txt) License.
