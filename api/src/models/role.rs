// api/src/models/role.rs
pub enum Role {
    Admin,
    ProjectManager,
    TeamMember,
}

impl Role {
    pub fn can_manage_users(&self) -> bool {
        match self {
            Role::Admin => true,
            _ => false,
        }
    }

    pub fn can_configure_app(&self) -> bool {
        match self {
            Role::Admin => true,
            _ => false,
        }
    }

    pub fn can_create_projects(&self) -> bool {
        true
    }

    // Add similar methods for other permissions
}
