# Optimizing your configuration

[1. Increase discovery TTL](#Increase-discovery-TTL)
[2. Increase collection interval](#Increase-collection-interval)
[3. Modify infrastructure agent configuration](#Modify-agent-configuration)
[4. Optimize TCP configuration](#Optimize-TCP-configuration)

## Increase discovery TTL

Time-To-Live of the cached discovery results, used to minimize the number of discovery processes. Define as a number followed by a time unit (s, m or h).

Examples: 30s, 10m, 1h, 0

Default: 1m

```yaml
---
discovery:
  ttl: 1m #<- increase this value
  command:
    exec: /var/db/newrelic-infra/nri-universal-discovery
    --------------- truncated -------------------
```

## Increase collection interval

The interval option sets the time between consecutive executions of an integration. The accepted format is an integer immediately followed by a time unit (s for seconds, m for minutes, h for hours).

The default is 30s, and the minimum accepted value is 15s. Any value lower than 15s is automatically set to 15s.

```yaml
---
discovery:
  ttl: 1m #<- NOT this value, look further below
  command:
    exec: /var/db/newrelic-infra/nri-universal-discovery
    env:
      NR_ACCOUNT_ID: "12345678"
      # other options...
integrations:
  - name: nri-flex
    interval: 1m ### <- adjust this value ###
    env:
      CONFIG_FILE: /etc/newrelic-infra/integrations.d/configs/netstat.yml
    --------------- truncated -------------------
```

## Modify infrastructure agent configuration

Modify the `/etc/newrelic-infra.yml` configuration file.

### Event Queue Depth

```
Two queues are used to send the events to metrics digest: (event -> eventQueue -> batch -> batchQueue -> HTTP post).

This config option allow us to increase the eventQueue size before accumulate these events in batches. Using this approach we minimize the impact of high-latency HTTP calls.

If HTTP calls are slow, we'll still be able to run the event queue receiver and accumulate a reasonable number of batches before we fill up on batches as well.

Default: 1000
```

Example

```yaml
license_key: yourLicenseKey
event_queue_depth: 2000 # <- add this option
```

### Batch Queue Depth

```
Two queues are used to send the events to metrics digest: (event -> eventQueue -> batch -> batchQueue -> HTTP post).

This config option allow us to increase the batchQueue size.
Default: 200
```

Example

```yaml
license_key: yourLicenseKey
batch_queue_depth: 300 # <- add this option
```

## Optimize TCP Configuration

If you are making a large amount of TCP connections it maybe worthwhile to tune your TCP configuration.

See the following articles for more detail:

- https://www.linkedin.com/pulse/ec2-tuning-1m-tcp-connections-using-linux-stephen-blum
- https://www.cyberciti.biz/faq/linux-tcp-tuning/
