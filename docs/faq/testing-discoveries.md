# Testing Discoveries

The easiest way to test the returned discoveries is to manually run the binary with your configured options.

You can do this via inline environment variables or the CLI flags.
For all available options see [configuration](/docs/configuration.md) or run `./nri-universal-discovery -h`.

Example:

```
NR_ACCOUNT_ID="12345" NR_API_KEY="NRAK-ABCD" NR_DECORATOR_FILE="examples/decoration_file.json" NR_QUERY="SELECT latest(datacenterName) as 'datacenterName', latest(vmConfigName) as 'vmName', latest(vmHostname) as 'vmHostname' FROM VSphereVmSample WHERE vmConfigName LIKE 'centos%'  FACET entityName LIMIT MAX" ./nri-universal-discovery
```
