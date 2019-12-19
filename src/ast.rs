#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode),
}

#[derive(Debug)]
pub enum Opcode {
    Add,
    Sub,
}
