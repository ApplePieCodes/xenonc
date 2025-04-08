use hashbrown::HashMap;
use crate::ast;
use crate::ast::{Tank};
use crate::hir::item::{Item, ItemId};
use crate::hir::module::Module;
use crate::resolve::symbol_table::Scope;

/// Also resolves symbols
pub fn lower_to_hir(table: Scope, tank: Tank) -> Module {
    let base = Module {
        name: tank.name,
        items: vec![]
    };
    
    let mut items: HashMap<ItemId, Item> = HashMap::new();
    
    for item in tank.items {
        match item {
            ast::Item::Module(m) => {
                
            }
        }
    }
    
    return base;
}