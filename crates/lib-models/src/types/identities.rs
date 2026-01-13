use sea_query::Iden;

#[derive(Iden)]
pub enum RoleIden {
	Id,
	Name,
	Description,
	Status,
}

#[derive(Iden)]
pub enum PermissionIden {
	Id,
	Module,
	Action,
	Level,
	Status,
}

#[derive(Iden)]
pub enum DepartmentIden {
	Id,
	Name,
	FullAddress,
	Description,
	Visible,
}
