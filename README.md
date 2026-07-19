<p align="center">
  <a href="https://github.com/studio2201">
    <img src="assets/header.jpg" alt="studio2201 banner" width="100%">
  </a>
</p>

# <img src="assets/icon.png" width="32" height="32" valign="middle"> Pulse

[![CI](https://github.com/studio2201/pulse/actions/workflows/ci.yml/badge.svg)](https://github.com/studio2201/pulse/actions/workflows/ci.yml)

Real-time system monitoring panel built in Rust.

## Quick Start

### Self-Hosting (Docker)
Pull and run the official Docker container:
```bash
docker run -d -p 4406:4406 -v /path/to/appdata:/app/data -v /proc:/host/proc:ro -v /sys:/sys:ro ghcr.io/studio2201/pulse:latest
```
