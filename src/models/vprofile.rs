use crate::models::profile::ProfileType;

use super::Profile;

/// VProfile enum representing different Veeam REST API profiles.
pub enum VProfile {
    /// Veeam Backup & Replication profile.
    VBR,
    /// Veeam Backup for Microsoft 365 profile.
    VB365,
    /// Veeam Backup for AWS profile.
    VBAWS,
    /// Veeam Backup for Azure profile.
    VBAZURE,
    /// Veeam Backup for Google Cloud Platform profile.
    VBGCP,
    /// Veeam ONE profile.
    VONE,
    /// Veeam Enterprise Manager profile.
    ENTMAN,
}

/// Implementation of methods for the VProfile enum.
impl VProfile {
    /// Get the profile data for the Veeam REST API.
    /// This method returns a `Profile` struct containing the profile information
    /// such as name, URL, port, API version, and X-API-Version.
    pub fn profile_data(&self) -> Profile {
        match self {
            VProfile::VB365 => Profile {
                profile_type: ProfileType::VB365,
                name: "VB365".to_string(),
                url: ":4443/v7/Token".to_string(),
                port: "4443".to_string(),
                api_version: "v8".to_string(),
                x_api_version: None,
            },
            VProfile::VBAWS => Profile {
                profile_type: ProfileType::VBAWS,
                name: "VBAWS".to_string(),
                url: ":11005/api/v1/token".to_string(),
                port: "11005".to_string(),
                api_version: "v1".to_string(),
                x_api_version: Some("1.7-rev0".to_string()),
            },
            VProfile::VBR => Profile {
                profile_type: ProfileType::VBR,
                name: "VBR".to_string(),
                url: ":9419/api/oauth2/token".to_string(),
                port: "9419".to_string(),
                api_version: "v1".to_string(),
                x_api_version: Some("1.2-rev1".to_string()),
            },
            VProfile::VBAZURE => Profile {
                profile_type: ProfileType::VBAZURE,
                name: "VBAZURE".to_string(),
                url: "/api/oauth2/token".to_string(),
                port: "".to_string(),
                api_version: "v8".to_string(),
                x_api_version: None,
            },
            VProfile::VBGCP => Profile {
                profile_type: ProfileType::VBGCP,
                name: "VBGCP".to_string(),
                url: ":13140/api/v1/token".to_string(),
                port: "13140".to_string(),
                api_version: "v1".to_string(),
                x_api_version: Some("1.4-rev0".to_string()),
            },
            VProfile::VONE => Profile {
                profile_type: ProfileType::VONE,
                name: "VONE".to_string(),
                url: ":1239/api/token".to_string(),
                port: "1239".to_string(),
                api_version: "v2.2".to_string(),
                x_api_version: None,
            },
            VProfile::ENTMAN => Profile {
                profile_type: ProfileType::ENTMAN,
                name: "ENTMAN".to_string(),
                url: ":9398/api/sessionMngr/?v=latest".to_string(),
                port: "9398".to_string(),
                api_version: "".to_string(),
                x_api_version: None,
            },
        }
    }
}
