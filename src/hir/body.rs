use uuid::Uuid;

pub type BodyId = Uuid;

pub struct Body {
    pub statements: Vec<Statement>
}