# Windows Remote Execution to collect WMI/WMIC metrics w/<b>winexe</b>

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

### WMIC Example

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
      CONFIG_FILE: /etc/newrelic-infra/integrations.d/universal-discovery-sub-configs/wmic-windows.yml
      ALLOW_ENV_COMMANDS: true
      FLEX_META: ${discovery.discoveryMeta}
      # build the remote command with winexe
      FLEX_CMD_PREPEND: "set +H && /bin/winexe -U ${discovery.user}%${discovery.pass} //${discovery.ip} "
      FLEX_CMD_WRAP: true
      STDIN_PIPE: true
```

[windows-wmic-metrics.yml](/examples/windows-wmic-metrics.yml)
Configuration location: `/etc/newrelic-infra/integrations.d/universal-discovery-sub-configs/<config name>.yml`

```yaml
name: WindowsWmicMetrics
custom_attributes:
  operatingSystem: windows
apis:
  - name: WmicSystem
    commands:
      - run: wmic cpu get loadpercentage,numberofcores,currentclockspeed,socketdesignation /format:csv
        split: horizontal
        regex_match: true
        row_start: 2
        # the below regex works on linux | does not work on mac
        split_by: (\S+),(\S+),(\S+),(\S+),(\S+\s\S+)
        set_header:
          - node
          - currentClockSpeed
          - loadPercentage
          - cores
          - socketDesignation
```

### WMI GetObject & gwmi Example

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
      CONFIG_FILE: /etc/newrelic-infra/integrations.d/universal-discovery-sub-configs/windows-wmiobj-metrics.yml
      ALLOW_ENV_COMMANDS: true
      FLEX_META: ${discovery.discoveryMeta}
      # build the remote command with winexe
      FLEX_CMD_PREPEND: "set +H && /bin/winexe -U ${discovery.user}%${discovery.pass} //${discovery.ip} "
      FLEX_CMD_WRAP: "'" # wrap with single quotes to simplify escaping the commands in the sub Flex config
      STDIN_PIPE: true
```

[windows-wmiobj-metrics.yml](/examples/windows-wmiobj-metrics.yml)
Configuration location: `/etc/newrelic-infra/integrations.d/universal-discovery-sub-configs/<config name>.yml`

```yaml
name: WindowsWmiMetrics
custom_attributes:
  operatingSystem: windows
#  Other available counters https://github.com/craignicholson/typeperf/blob/master/counters.txt
apis:
  - name: Cpu
    commands:
      - run: powershell /c Get-WmiObject win32_processor | Measure-Object -property LoadPercentage -Average | Select Average
        split_output: Average
        regex_matches:
          - expression: (\S+)
            keys:
              - cpuPercent
    merge: WmiSystemSample
  - name: Memory
    commands:
      - run: powershell gwmi -Class win32_operatingsystem | Select-Object @{Name = \"MemoryUsage\"; Expression = { \"{0:N2}\" -f ((($_.TotalVisibleMemorySize - $_.FreePhysicalMemory)*100)/ $_.TotalVisibleMemorySize) }}
        split_output: Memory
        regex_matches:
          - expression: (\S+)
            keys:
              - memoryPercent
    merge: WmiSystemSample
```
