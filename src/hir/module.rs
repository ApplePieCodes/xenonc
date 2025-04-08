use crate::hir::item::ItemId;

pub struct Module {
    pub name: String,
    pub items: Vec<ItemId>
}