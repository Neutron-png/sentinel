use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! define_selectable {
    ($name:ident, $($variant:ident => $label:literal),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum $name {
            $($variant),+
        }

        impl $name {
            pub const ALL: &'static [$name] = &[$($name::$variant),+];

            pub fn label(&self) -> &'static str {
                match self {
                    $($name::$variant => $label),+
                }
            }

            pub fn from_label(label: &str) -> Option<Self> {
                match label {
                    $($label => Some($name::$variant),)+
                    _ => None,
                }
            }

            pub fn next(&self) -> Self {
                let all = Self::ALL;
                let pos = all.iter().position(|e| e == self).unwrap_or(0);
                all[(pos + 1) % all.len()]
            }

            pub fn prev(&self) -> Self {
                let all = Self::ALL;
                let pos = all.iter().position(|e| e == self).unwrap_or(0);
                all[(pos + all.len() - 1) % all.len()]
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.label())
            }
        }
    };
}

define_selectable!(Environment,
    Production => "Production",
    Staging => "Staging",
    Development => "Development",
);

define_selectable!(Scope,
    WebApplication => "Web Application",
    Api => "API",
    MobileBackend => "Mobile Backend",
    Network => "Network",
);

define_selectable!(Methodology,
    Owasp => "OWASP",
    Ptes => "PTES",
    Osstmm => "OSSTMM",
    Custom => "Custom",
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssessmentStatus {
    Draft,
    Active,
    Completed,
    Archived,
}

impl AssessmentStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::Active => "Active",
            Self::Completed => "Completed",
            Self::Archived => "Archived",
        }
    }
}

impl std::fmt::Display for AssessmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

impl std::str::FromStr for AssessmentStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Draft" => Ok(Self::Draft),
            "Active" => Ok(Self::Active),
            "Completed" => Ok(Self::Completed),
            "Archived" => Ok(Self::Archived),
            _ => Err(format!("unknown assessment status: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assessment {
    pub id: Uuid,
    pub name: String,
    pub target: String,
    pub environment: Environment,
    pub scope: Scope,
    pub methodology: Methodology,
    pub status: AssessmentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
