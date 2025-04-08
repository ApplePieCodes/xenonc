use uuid::Uuid;
use crate::hir::function::Function;
use crate::hir::module::Module;
use crate::hir::r#enum::Enum;
use crate::hir::r#struct::Struct;

pub type ItemId = Uuid;

pub enum Item {
    Variable(Variable),
    Function(Function),
    Module(Module),
    Enum(Enum),
    Struct(Struct),
    Trait(Trait),
}