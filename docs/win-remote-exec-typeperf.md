# Windows Remote Execution to collect typeperf metrics w/<b>winexe</b>

## About

Winexe is a GNU/Linux based application that allows users to execute commands remotely on Windows systems. It installs a service on the remote system, executes the command and uninstalls the service. Winexe allows execution of most of the windows shell commands.

### Requirements:

- Linux host
- <b>winexe</b> installed
  - [Existing packages](https://software.opensuse.org/download/package?package=winexe&project=home%3Auibmz%3Awinexe)
  - Resources for building and installing manually (advanced)
    - https://www.secpod.com/blog/winexe/
    - https://github.com/skalkoto/winexe
- Disable UAC Remote Restrictions (For Vista,7,8,10 and above)
  - [How to disable UAC remote restrictions](https://docs.microsoft.com/en-us/troubleshoot/windows-server/windows-security/user-account-control-and-remote-restriction#how-to-disable-uac-remote-restrictions)
  - Apply the above via GroupPolicy across all your systems to make this as easy as possible

Use a decorator file to help apply credentials or any other needed meta.
[Decorator File](./decorator-file.md)

[Example Netstat Flex Config](/examples/netstat-windows.yml)

---

## Manually testing commands

```
List all options, run:
winexe

Example command structure:
winexe -U [Domain/]User%Password //host command

Examples:
winexe -U HOME/Administrator%Pass123 //192.168.0.1 “netstat -a”
winexe -U HOME/Administrator%Pass123 //192.168.0.1 “ipconfig -all”
winexe -U HOME/Administrator%Pass123 //192.168.0.1 “ping localhost”
winexe -U myUser%myPass! //192.168.0.243 ipconfig

To launch a windows shell from inside your Linux box:
winexe -U HOME/Administrator%Pass123 //192.168.0.1 “cmd.exe”

Debugging commands:
Use the -d flag and set a value from 1 being lowest to 11 as the highest.

winexe -d 1 -U myUser%myPass! //192.168.0.243 ipconfig
```

---

## Example with Discovery File

windows-discoveries.json

```json
{
  "defaults": {
    "user": "kav",
    "pass": "p@ass!"
  },
  "discovery_items": [
    {
      "ip": "192.168.0.243",
      "team": "batman"
    }
  ]
}
```

Configuration location: `/etc/newrelic-infra/integrations.d/<config name>.yml`

```yaml
---
---
discovery:
  ttl: 1m
  command:
    exec: /var/db/newrelic-infra/nri-universal-discovery # path to discovery binary
    env:
      NR_DISCOVERY_FILE: /etc/newrelic-infra/integrations.d/windows-discovery-file.json
      NR_META_WHITELIST: "ip,team"
    match:
      ip: /\S+/ # match is required and accepts regex when enclosed between forward slashes eg. /<regex>/
integrations:
  - name: nri-flex
    interval: 1m
    env:
      # path to Flex config
      CONFIG_FILE: /etc/newrelic-infra/integrations.d/universal-discovery-sub-configs/windows-typeperf-metrics.yml
      ALLOW_ENV_COMMANDS: true
      FLEX_META: ${discovery.discoveryMeta}
      # build the remote command with winexe
      FLEX_CMD_PREPEND: "set +H && /bin/winexe -U ${discovery.user}%${discovery.pass} //${discovery.ip} "
      FLEX_CMD_WRAP: true
      STDIN_PIPE: true
```

[windows-typeperf-metrics.yml](/examples/windows-typeperf-metrics.yml)
Configuration location: `/etc/newrelic-infra/integrations.d/universal-discovery-sub-configs/<config name>.yml`

```yaml
name: WindowsTypePerfMetrics
custom_attributes:
  operatingSystem: windows
#  Other available counters https://github.com/craignicholson/typeperf/blob/master/counters.txt
apis:
  - name: System
    commands:
      # intentionally accessing powershell like this rather then setting via shell so this can be used as a remote integration automatically
      - run: powershell /c typeperf -sc 1 '\Processor(_total)\% Processor Time' '\Memory\Committed Bytes' '\Memory\Available Bytes' '\LogicalDisk(_total)\% Free Space' '\LogicalDisk(_total)\Free Megabytes' '\Network Interface(*)\Bytes Received/sec' '\Network Interface(*)\Bytes Sent/sec'
        split_output: Processor
        regex_matches:
          - expression: .+,\"(\d+.\d+)\",\"(\d+.\d+)\",\"(\d+.\d+)\",\"(\d+.\d+)\",\"(\d+.\d+)\",\"(\d+.\d+)\",\"(\d+.\d+)\"
            keys:
              [
                cpuPercent,
                memoryCommittedBytes,
                memoryAvailableBytes,
                logicalDiskFreeSpacePercent,
                logicalDiskFreeMegabytes,
                networkInterfaceBytesRecievedPerSec,
                networkInterfaceBytesSentsPerSec,
              ]
```
