use crate::hir::item::Item;

/// Reference by a type id
pub struct Struct {
    pub name: String,
    pub fields: Vec<Item>
}