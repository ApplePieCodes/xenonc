use crate::hir::typeid::TypeId;

/// Reference by a type id
pub struct Enum {
    pub name: String,
    pub variants: Vec<Variant>
}

pub struct Variant {
    pub name: String,
    pub r#type: Option<TypeId>,
}