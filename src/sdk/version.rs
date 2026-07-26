#![allow(dead_code)]

pub const SDK_VERSION: &str = "2.0.0";

pub struct VersionChecker;

impl VersionChecker {
    pub fn is_compatible(sdk_version: &str, required: &str) -> bool {
        let parts: Vec<&str> = required.split('.').collect();
        let sdk_parts: Vec<&str> = sdk_version.split('.').collect();
        if parts.len() >= 2 && sdk_parts.len() >= 2 {
            parts[0] == sdk_parts[0] && parts[1] <= sdk_parts[1]
        } else {
            false
        }
    }

    pub fn check_sentinel_version(
        plugin_min: &str,
        plugin_max: Option<&str>,
        sentinel: &str,
    ) -> bool {
        let pmin = parse_semver(plugin_min);
        let current = parse_semver(sentinel);
        if let Some(pmax) = plugin_max {
            let pmax_ver = parse_semver(pmax);
            current >= pmin && current <= pmax_ver
        } else {
            current >= pmin
        }
    }
}

fn parse_semver(v: &str) -> (u32, u32, u32) {
    let parts: Vec<u32> = v.split('.').filter_map(|s| s.parse().ok()).collect();
    (
        parts.first().copied().unwrap_or(0),
        parts.get(1).copied().unwrap_or(0),
        parts.get(2).copied().unwrap_or(0),
    )
}
