pub type Program = Vec<Stmt>;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    #[allow(clippy::enum_variant_names)]
    BlockStmt(BlockStmt),
    FuncDecl(FuncDecl),
    #[allow(clippy::enum_variant_names)]
    IfStmt(IfStmt),
    #[allow(clippy::enum_variant_names)]
    ForStmt(ForStmt),
    #[allow(clippy::enum_variant_names)]
    VarStmt(VarStmt),
    #[allow(clippy::enum_variant_names)]
    ExprStmt(ExprStmt),
    #[allow(clippy::enum_variant_names)]
    ContinueStmt,
    #[allow(clippy::enum_variant_names)]
    BreakStmt,
    #[allow(clippy::enum_variant_names)]
    ReturnStmt(ReturnStmt),
}
pub type BlockStmt = Vec<Stmt>;

#[derive(Debug, PartialEq, Clone)]
pub struct FuncDecl {
    pub ident_func: Ident,
    pub ident_param: Vec<Ident>,
    pub block: BlockStmt,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStmt {
    pub condition: Expr,
    pub block: BlockStmt,
    pub else_clause: Option<IfStmtElseClause>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum IfStmtElseClause {
    IfStmtBlock(BlockStmt),
    IfStmt(Box<IfStmt>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForStmt {
    pub init: Option<ForStmtInit>,
    pub cond: Option<ForStmtCond>,
    pub afterthought: Option<ForStmtAfterthought>,
    pub block: BlockStmt,
}
#[derive(Debug, PartialEq, Clone)]
pub enum ForStmtInit {
    Var(VarStmt),
    Expr(Expr),
}
pub type ForStmtCond = Expr;
pub type ForStmtAfterthought = Expr;

#[derive(Debug, PartialEq, Clone)]
pub struct VarStmt {
    pub ident: Ident,
    pub expr: Option<Expr>,
}

pub type ExprStmt = Expr;

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStmt {
    pub expr: Option<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Literal(Literal),
    Ident(Ident),
    PrefixOp {
        kind: PrefixOpKind,
        rhs: Box<Expr>,
    },
    InfixOp {
        kind: InfixOpKind,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    PostfixOp {
        kind: PostfixOpKind,
        lhs: Box<Expr>,
    },
}
impl Expr {
    // Constructors for the conviniences of tests
    #[allow(dead_code)]
    pub fn literal_bool(bool: bool) -> Self {
        Self::Literal(Literal::Bool(bool))
    }
    #[allow(dead_code)]
    pub fn literal_int(int: u64) -> Self {
        Self::Literal(Literal::Int(int))
    }
    #[allow(dead_code)]
    pub fn literal_string(string: String) -> Self {
        Self::Literal(Literal::String(string))
    }
    #[allow(dead_code)]
    pub fn literal_array(array: Vec<Expr>) -> Self {
        Self::Literal(Literal::Array(array))
    }
    #[allow(dead_code)]
    pub fn literal_none() -> Self {
        Self::Literal(Literal::None)
    }
    #[allow(dead_code)]
    pub fn ident(name: &str) -> Self {
        Self::Ident(Ident {
            name: name.to_string(),
        })
    }
    #[allow(dead_code)]
    pub fn prefix(kind: PrefixOpKind, rhs: Expr) -> Self {
        Self::PrefixOp {
            kind,
            rhs: Box::new(rhs),
        }
    }
    #[allow(dead_code)]
    pub fn infix(kind: InfixOpKind, lhs: Expr, rhs: Expr) -> Self {
        Self::InfixOp {
            kind,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrefixOpKind {
    Pos,
    Neg,
    Not,
}

#[derive(Debug, PartialEq, Clone)]
pub enum InfixOpKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Eq,
    NotEq,
    Gt,
    Ge,
    Lt,
    Le,
    And,
    Or,
    Assign,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PostfixOpKind {
    Index(Box<Expr>),
    Call(Vec<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Bool(bool),
    Int(u64),
    String(String),
    Array(Vec<Expr>),
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ident {
    pub name: String, // TODO: Consider changing to `&str`
}
