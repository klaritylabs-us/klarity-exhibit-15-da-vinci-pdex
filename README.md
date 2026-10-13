# Da Vinci PDex / member attribution — Rust

Small library and CLI that sanity-check **Coverage** JSON used in **Da Vinci PDex**-style member attribution flows before files leave the payer boundary: resource type and **`beneficiary.reference`** linking coverage to a member.

Synthetic fixtures only. Not ONC certification, not a substitute for Da Vinci PDex test scripts or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 15** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [Da Vinci PDex](http://hl7.org/fhir/us/davinci-pdex/)

## What it checks

- `resourceType` is **`Coverage`**
- **`beneficiary.reference`** present

| Problem | What breaks |
|--------|-------------|
| **Patient-only export** | Payer attribution missing Coverage wrapper |
| **Orphan Coverage** | Cannot resolve member in downstream attribution |
| **Beneficiary as inline resource only** | Partner systems expect resolvable reference |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-pdex -- examples/valid-coverage.json
cargo run --bin validate-pdex -- examples/invalid-pdex.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or JSON.

## Sample output

One command:

```bash
cargo run --bin validate-pdex -- examples/valid-coverage.json
```

**Pass** (`examples/valid-coverage.json`) — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-pdex.json`) — exit code `1` (issues array; more entries may appear):

```json
[
  {
    "severity": "error",
    "path": "resourceType",
    "detail": "PDex attribution payloads often center Coverage",
    "rule_id": "pdex-coverage"
  }
]
```


## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken Coverage payloads for tests and demos

## Inferno vs this repo

Da Vinci **PDex** suites exercise payer–provider exchange on live endpoints. This repo is **attribution payload shape** in your mapper before files leave the payer boundary.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
