[![New Relic Experimental header](https://github.com/newrelic/opensource-website/raw/master/src/images/categories/Experimental.png)](https://opensource.newrelic.com/oss-category/#new-relic-experimental)

# nri-universal-discovery

Dynamically create discovery options from the following sources:

- NRQL
- Entities
- Files

## Installation

_TO DO_

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

## Building

_TO DO_

## Testing

_TO DO_

## Support

New Relic has open-sourced this project. This project is provided AS-IS WITHOUT WARRANTY OR DEDICATED SUPPORT. Issues and contributions should be reported to the project here on GitHub. We encourage you to bring your experiences and questions to the [Explorers Hub](https://discuss.newrelic.com) where our community members collaborate on solutions and new ideas.

## Contributing

We encourage your contributions to improve nri-universal-discovery! Keep in mind when you submit your pull request, you'll need to sign the CLA via the click-through using CLA-Assistant. You only have to sign the CLA one time per project. If you have any questions, or to execute our corporate CLA, required if your contribution is on behalf of a company, please drop us an email at opensource@newrelic.com.

**A note about vulnerabilities**

As noted in our [security policy](../../security/policy), New Relic is committed to the privacy and security of our customers and their data. We believe that providing coordinated disclosure by security researchers and engaging with the security community are important means to achieve our security goals.

If you believe you have found a security vulnerability in this project or any of New Relic's products or websites, we welcome and greatly appreciate you reporting it to New Relic through [HackerOne](https://hackerone.com/newrelic).

## License

nri-universal-discovery is licensed under the [Apache 2.0](http://apache.org/licenses/LICENSE-2.0.txt) License.
