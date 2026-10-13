# SMART Backend Services (JWT shape) — Rust

Small library and CLI that sanity-check **backend service JWT** wire format before you send system-to-system SMART requests: three non-empty segments, base64url-style header — structural checks only, not signature verification or full claim semantics.

Synthetic tokens only. Not a security audit tool, not ONC certification or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 11** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [SMART Backend Services](http://hl7.org/fhir/smart-app-launch/backend-services.html)

## What it checks

- Token is **`header.payload.signature`** (exactly three dot-separated parts)
- No empty segment
- Warning if header segment does not start with **`eyJ`** (typical base64url JSON header)

| Problem | What breaks |
|--------|-------------|
| **Two-part “JWT”** | Authorization middleware rejects before FHIR call |
| **Truncated signature** | Opaque 401/403 from token endpoint |
| **Non-JWT bearer string** | Retry loops against SMART backend routes |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-smart-backend -- examples/valid-jwt.txt
cargo run --bin validate-smart-backend -- examples/invalid-jwt.txt
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or input.

## Sample output

One command:

```bash
cargo run --bin validate-smart-backend -- examples/valid-jwt.txt
```

**Pass** (`examples/valid-jwt.txt`) — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-jwt.txt`) — exit code `1` (issues array; more entries may appear):

```json
[
  {
    "severity": "error",
    "path": "jwt",
    "detail": "SMART backend JWT should have header.payload.signature",
    "rule_id": "jwt-shape"
  }
]
```


## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken token lines for tests and demos

## Inferno vs this repo

Inferno and SMART suites exercise **authorized backend flows** on live systems. This repo is **token shape** — catching malformed JWTs in config or secret stores before they hit the wire.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
