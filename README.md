# ipgeomancer

ipgeomancer is a collection of Rust libraries and tools for the internet / IP ecosystem.

**NOTE**: This project is in early development, and many parts of it are immature or incomplete.

It has three main goals:

* libraries, all-in-one CLI and (web) UI for investigating "the internet", useful for
  anyone working on network infrastructure
  (DNS, WHOIS, RDAP, TLS certificates, RIR data, BGP, ping, traceroute, MX servers, ...)

* efficient IP geo-location system that does not depend on APIs or expensive databases
  
* provide a database for storing and querying data about "the internet", such as RIR data, DNS, BGP, ...

The web service is publically available at: https://ipgeomancer.condacity.io/


## CLI

### Installation

Since the crates are not yet published on crates.io, you can install the CLI directly from the repository:

```bash
cargo install --locked --git https://github.com/theduke/ipgeomancer ipgeom_cli
```

The `ipgeom` command line tool provides:

* DNS queries with `ipgeom dns query`
* WHOIS lookups with `ipgeom whois`
* RDAP queries with `ipgeom rdap`
* Ping hosts with `ipgeom ping`
* Trace network paths with `ipgeom traceroute`
* RIR database and geolocation db generation:
  - Fetch database dumps for RIRs (RIPE, ARIN, APNIC, LACNIC, AFRINIC)
  - Ingest RIR RPSL data into a database
  - Generate a geoip2/mmdb geolocation database from RIR data

  (see `ipgeom store` subcommands)
* RPSL dump parsing/printing/conversions with `ipgeom rpsl print`

* HTTP web server that exposes a REST API as well as a web UI for the above functionality:
  `ipgeom server` (add `--open` to automatically launch the site in your browser)
  

## Libraries

| Crate | Functionality |
|-------|---------------|
| **ipgeom_rpsl** | Streaming parser for RPSL and typed representation of well-known RPSL objects. |
| **ipgeom_rir** | Regional Internet Registry (RIR) database, ip geolocation system, ... |
| **ipgeom_whois** | Asynchronous WHOIS client with referral following. |
| **ipgeom_query** | Convenience wrappers for DNS, RDAP and WHOIS queries. |


## License

This project is licensed under the terms of the MIT License. See [LICENSE](LICENSE) for details.
