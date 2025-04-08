use std::hash::{DefaultHasher, Hash, Hasher};
use crate::ast::{Function, Item, Module, Tank, Type};

#[derive(Default)]
pub struct Scope {
    pub symbol: Vec<Symbol>,
    pub scopes: Vec<Scope>,
}

#[derive(Default)]
pub struct Symbol {
    pub name: String,
    pub mangled: String,
    pub data: SymbolData,
}

#[derive(Default)]
pub enum SymbolData {
    Module(Scope),
    Variable(Option<Type>),
    Function(Scope, Type),
    #[default]
    Null,
}

pub fn walk_tank(tank: Tank) -> Scope {
    let mut scope = Scope::default();
    
    for item in tank.items {
        scope.symbol.push(parse_item(item));
    }
    
    return scope;
}

pub fn parse_module_scope(module: Module) -> SymbolData {
    let mut scope = Scope::default();
    
    for item in module.items.unwrap() {
        scope.symbol.push(parse_item(item));
    }
    
    return SymbolData::Module(scope);
}

pub fn parse_item(item: Item) -> Symbol {
    let mut symbol = Symbol::default();

    match item {
        Item::Module(m) => {
            let mut hash = DefaultHasher::new();
            m.hash(&mut hash);
            symbol.mangled = format!("_Xe{:x}", hash.finish());
            symbol.name = symbol.name.clone();
            symbol.data = parse_module_scope(m);
        }
        Item::Variable(v) => {
            let mut hash = DefaultHasher::new();
            v.hash(&mut hash);
            symbol.mangled = format!("_Xe{:x}", hash.finish());
            symbol.name = symbol.name.clone();
            symbol.data = SymbolData::Variable(v.r#type);
        }
        Item::Function(f) => {
            let mut hash = DefaultHasher::new();
            f.hash(&mut hash);
            symbol.mangled = format!("_Xe{:x}", hash.finish());
            symbol.name = symbol.name.clone();
            symbol.data = parse_function_scope(f);
        }
    }
    
    return symbol;
}

pub fn parse_function_scope(function: Function) -> SymbolData {
    let mut scope = Scope::default();
    
    
    return SymbolData::Function(scope, function.output);
}