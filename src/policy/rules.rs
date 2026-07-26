#![allow(dead_code)]

use crate::policy::models::RuleControl;

pub fn is_rule_enabled(control: &RuleControl, rule_id: &str) -> bool {
    if control.disabled_rules.contains(&rule_id.to_string()) {
        return false;
    }
    if let Some(ov) = control.rule_overrides.iter().find(|o| o.rule_id == rule_id) {
        if let Some(en) = ov.enabled {
            return en;
        }
    }
    control.enabled_rules.contains(&rule_id.to_string()) || control.enabled_rules.is_empty()
}

pub fn get_rule_priority(control: &RuleControl, rule_id: &str) -> u8 {
    control
        .rule_overrides
        .iter()
        .find(|o| o.rule_id == rule_id)
        .and_then(|o| o.priority)
        .unwrap_or(control.default_priority)
}

pub fn get_group_rules(control: &RuleControl, group_name: &str) -> Vec<String> {
    control
        .rule_groups
        .iter()
        .find(|g| g.name == group_name)
        .map(|g| g.rule_ids.clone())
        .unwrap_or_default()
}
