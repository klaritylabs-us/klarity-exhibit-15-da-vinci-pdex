use serde_json::Value;
use crate::outcome::{ValidationIssue, ValidationReport};

pub fn validate_pdex_json(json: &str) -> Result<ValidationReport, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("JSON: {e}"))?;
    let mut r = ValidationReport::ok();
    if v.get("resourceType").and_then(|x| x.as_str()) != Some("Coverage") {
        r.push(ValidationIssue::error("resourceType", "PDex attribution payloads often center Coverage", "pdex-coverage"));
    }
    if v.get("beneficiary").and_then(|b| b.get("reference")).is_none() {
        r.push(ValidationIssue::error("beneficiary", "Coverage.beneficiary reference expected", "beneficiary"));
    }
    Ok(r)
}
