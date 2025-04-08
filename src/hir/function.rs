use crate::hir::body::BodyId;
use crate::hir::item::ItemId;

pub struct Function {
    pub name: String,
    pub body: Option<BodyId>
}