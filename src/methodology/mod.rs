pub mod wstg;

#[derive(Debug, Clone)]
pub struct MethodologyDefinition {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub version: String,
    #[allow(dead_code)]
    pub description: String,
    pub sections: Vec<MethodologySection>,
}

#[derive(Debug, Clone)]
pub struct MethodologySection {
    pub title: String,
    pub activities: Vec<MethodologyActivity>,
}

#[derive(Debug, Clone)]
pub struct MethodologyActivity {
    pub title: String,
    pub description: String,
    pub tasks: Vec<MethodologyTask>,
}

#[derive(Debug, Clone)]
pub struct MethodologyTask {
    pub title: String,
    pub description: String,
    pub reference: String,
}

pub fn load_definition(methodology_name: &str) -> Option<MethodologyDefinition> {
    match methodology_name.to_lowercase().as_str() {
        "owasp" => Some(wstg::owasp_definition()),
        _ => None,
    }
}
