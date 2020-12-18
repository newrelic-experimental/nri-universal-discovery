# Manually updating Flex to the latest

If you require a newer versions of Flex that has not yet been updated with the agent bundle you can manually update as per below instructions.

1. Download the latest release from https://github.com/newrelic/nri-flex/releases for your target system.
2. Extract the package eg.

```
tar -xvf tar -xvf nri-flex_<version>.tar.gz
```

3. Move the Flex binary to the appropriate location
   eg.

```
sudo mv nri-flex /var/db/newrelic-infra/newrelic-integrations/bin/
```

## Checking if the version has updated

Execute

```
/var/db/newrelic-infra/newrelic-integrations/bin/nri-flex
```

Output

```
 /tmp/nri-flex
INFO[0000] com.newrelic.nri-flex GOARCH=amd64 GOOS=linux version=1.3.8
---------- truncated ----------
```
