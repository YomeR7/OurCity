impl From<api::common::BuildingKind> for crate::entities::sea_orm_active_enums::BuildingKind {
    fn from(value: api::common::BuildingKind) -> Self {
        match value {
            api::common::BuildingKind::House => crate::entities::sea_orm_active_enums::BuildingKind::House,
        }
    }
}

impl From<crate::entities::sea_orm_active_enums::BuildingKind> for api::common::BuildingKind {
    fn from(value: crate::entities::sea_orm_active_enums::BuildingKind) -> Self {
        match value {
            crate::entities::sea_orm_active_enums::BuildingKind::House => api::common::BuildingKind::House,
        }
    }
}
