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
        match self {
            Role::Admin | Role::ProjectManager => true,
            _ => false,
        }
    }

    // Add similar methods for other permissions
}
