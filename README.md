# Foggler
🕸️ Foggler keeps a constant watch on Dead by Daylight server connections, peeking through the mist to ensure you're always in the know. 

## Download
- Go in [Release](https://github.com/Neotoxic-off/Foggler/releases)
- Download latest version for your device

## Usage
```SH
🕸️ Foggler keeps a constant watch on Dead by Daylight server connections, peeking through the mist to ensure you're always in the know

Usage: foggler [OPTIONS] --servers <SERVERS>

Options:
  -s, --servers <SERVERS>          
  -t, --timeout <TIMEOUT>          [default: 3]
  -p, --port-socket <PORT_SOCKET>  [default: 443]
      --debug                      
  -h, --help                       Print help
  -V, --version                    Print version
```

## Example
```SH
./foggler --servers servers.toml
```
```LOG
[2025-04-22T11:06:34Z INFO  foggler::core] AP East 1................: 226ms
[2025-04-22T11:06:34Z INFO  foggler::core] AP South 1...............: 120ms
[2025-04-22T11:06:34Z INFO  foggler::core] AP North East 1..........: 242ms
[2025-04-22T11:06:34Z INFO  foggler::core] AP North East 2..........: 245ms
[2025-04-22T11:06:34Z INFO  foggler::core] AP South East 1..........: 176ms
[2025-04-22T11:06:34Z INFO  foggler::core] AP South East 2..........: 283ms
[2025-04-22T11:06:34Z INFO  foggler::core] CA 1.....................: 124ms
[2025-04-22T11:06:34Z INFO  foggler::core] EU Central 1.............: 40ms
[2025-04-22T11:06:34Z INFO  foggler::core] EU West 1................: 51ms
[2025-04-22T11:06:34Z INFO  foggler::core] EU West 2................: 42ms
[2025-04-22T11:06:34Z INFO  foggler::core] SA East..................: 218ms
[2025-04-22T11:06:34Z INFO  foggler::core] US East 1................: 105ms
[2025-04-22T11:06:34Z INFO  foggler::core] US East 2................: 116ms
[2025-04-22T11:06:34Z INFO  foggler::core] US West 1................: 162ms
[2025-04-22T11:06:34Z INFO  foggler::core] US West 2................: 174ms
```

## Rebuild
```SH
git clone git@github.com:Neotoxic-off/Foggler.git
cd Flogger/flogger
cargo build
```

## Servers
```TOML
[[servers]]
name = "AP East 1"
url = "ec2.ap-east-1.amazonaws.com"

[[servers]]
name = "AP South 1"
url = "gamelift.ap-south-1.amazonaws.com"

[[servers]]
name = "AP North East 1"
url = "gamelift.ap-northeast-1.amazonaws.com"

[[servers]]
name = "AP North East 2"
url = "gamelift.ap-northeast-2.amazonaws.com"

[[servers]]
name = "AP South East 1"
url = "gamelift.ap-southeast-1.amazonaws.com"

[[servers]]
name = "AP South East 2"
url = "gamelift.ap-southeast-2.amazonaws.com"

[[servers]]
name = "CA 1"
url = "gamelift.ca-central-1.amazonaws.com"

[[servers]]
name = "EU Central 1"
url = "gamelift.eu-central-1.amazonaws.com"

[[servers]]
name = "EU West 1"
url = "gamelift.eu-west-1.amazonaws.com"

[[servers]]
name = "EU West 2"
url = "gamelift.eu-west-2.amazonaws.com"

[[servers]]
name = "SA East"
url = "gamelift.sa-east-1.amazonaws.com"

[[servers]]
name = "US East 1"
url = "gamelift.us-east-1.amazonaws.com"

[[servers]]
name = "US East 2"
url = "gamelift.us-east-2.amazonaws.com"

[[servers]]
name = "US West 1"
url = "gamelift.us-west-1.amazonaws.com"

[[servers]]
name = "US West 2"
url = "gamelift.us-west-2.amazonaws.com"
```
