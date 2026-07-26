#![allow(dead_code)]

use crate::reporting::models::Branding;

pub struct BrandingEngine;

impl BrandingEngine {
    pub fn default_branding() -> Branding {
        Branding::default()
    }

    pub fn apply(template: &str, branding: &Branding) -> String {
        template
            .replace("{{company}}", &branding.company_name)
            .replace("{{consultant}}", &branding.consultant_name)
            .replace("{{footer}}", &branding.footer_text)
            .replace("{{color}}", &branding.primary_color)
    }
}
