<div align="center">
  <h1>🕸️ Foggler</h1>

  <img src="https://img.shields.io/github/v/release/Neotoxic-off/foggler" alt="Github release">
  <img src="https://img.shields.io/github/last-commit/Neotoxic-off/foggler" alt="GitHub last commit">
  <img src="https://img.shields.io/github/repo-size/Neotoxic-off/foggler" alt="GitHub repo size">
  <img src="https://img.shields.io/github/languages/top/Neotoxic-off/foggler" alt="GitHub top language">

  <p><strong>Foggler keeps a constant watch on Dead by Daylight server connections, peeking through the fog</strong></p>
</div>


<div align="center">
  <img src="./images/avatar.png" alt="Foggler Avatar" width="240" style="border-radius: 12px; margin-top: 20px;"/>
</div>

---

## ℹ️ About

- This project is intended to be deployed on a server for monitoring via **Loki & Grafana**.
- Running it as a binary on your personal device might be overkill. Check the arguments first.
- Foggler fetches the servers one by one trying to reach them to determinate the latency

---

## 🐳 Container

### Requirements
- Docker

### Stack
- Grafana
- Loki
- Promtail
- Rust

### Environment Variables
```ini
GRAFANA_USERNAME=admin
GRAFANA_PASSWORD=i_am_not_stupid_to_set_admin_as_password
PORT=443
TIMEOUT=3
WAIT=600
```

### Deploy
[deploy.sh](./deploy.sh)

```sh
chmod +x deploy.sh
./deploy.sh
```

---

## 📊 Grafana

### Suggested Panel Setting
> **Graph styles** → Connect null values → Always

```sh
{job="foggler"}
| json
```

<p align="center">
  <img src="images/image.png" width="80%" height="80%"/>
</p>

---

## 🔧 Binary

### Usage
```sh
Foggler keeps a constant watch on Dead by Daylight server connections

Usage: foggler [OPTIONS] --servers <SERVERS>

Options:
  -s, --servers <SERVERS>  List of servers to ping
  -t, --timeout <TIMEOUT>  Time limit to wait before timeout [default: 3]
  -p, --port <PORT>        Port to ping [default: 443]
  -w, --wait <WAIT>        Waiting time in sec between checks (0 for one-time) [default: 600]
  -l, --logs <LOGS>        Logs folder for monitoring [default: logs]
  -h, --help               Print help
  -V, --version            Print version
```

### Example
```sh
./foggler --servers servers.toml --wait 0
```

### Output
```log
2025-04-23T10:11:00.196815850+00:00  INFO foggler::core: server="AP East 1" ping="236ms"
2025-04-23T10:11:00.196931278+00:00  INFO foggler::core: server="AP South 1" ping="121ms"
...
2025-04-23T10:11:00.197740966+00:00  INFO foggler::core: server="US West 2" ping="185ms"
```

---

## 🛠️ Rebuild
```sh
git clone git@github.com:Neotoxic-off/Foggler.git
cd Flogger/flogger
cargo build
```

---

## 🌐 Servers

[servers.toml](./servers.toml)

