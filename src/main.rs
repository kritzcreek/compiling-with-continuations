// The Lambda language (IR between SML and CPS)

type Var = String;

type AccessPath = Vec<i32>; // Must be nonempty

#[derive(PartialEq, Debug)]
enum ConRep {
    Undecided,
    Tagged(u32),
    Constant(u32),
    Transparent,
    TransU,
    TransB,
    Ref,
    Variable(Var, AccessPath),
    VariableC(Var, AccessPath),
}

#[derive(PartialEq, Debug)]
enum Con {
    Data(ConRep),
    Int(i32),
    Real(String),
    String(String),
}

#[derive(PartialEq, Debug)]
enum PrimOp {
    Add,
    Mul,
    Minus,
    Div,
    IEql,
    InEql,
    Lt,
    Lte,
    Gt,
    Gte,
    RangeCheck,
    Not,
    Subscript,
    OrdOf,
    Assign,
    UnboxedAssign,
    Update,
    UnboxedUpdate,
    Store,
    MakeRef,
    MakeRefUnboxed,
    ALength,
    SLength,
    GetHandler,
    SetHandler,
    Boxed,
    FAdd,
    FSub,
    FMul,
    FDiv,
    FEql,
    FInEql,
    FLt,
    FLtE,
    FGt,
    FGtE,
    RShift,
    LShift,
    Orb,
    AndB,
    XorB,
    NotB,
}

#[derive(PartialEq, Debug)]
enum Exp {
    Var(Var),
    Fn(Var, Box<Exp>),
    Fix(Vec<Var>, Vec<Exp>, Box<Exp>),
    App(Box<Exp>, Box<Exp>),
    Int(i32),
    Real(String),
    String(String),
    Switch(Box<Exp>, Vec<ConRep>, Vec<(Con, Exp)>, Option<Box<Exp>>),
    Con(ConRep, Box<Exp>),
    Decon(ConRep, Box<Exp>),
    Record(Vec<Exp>),
    Select(i32, Box<Exp>),
    Raise(Box<Exp>),
    Handle(Box<Exp>, Box<Exp>),
    Prim(PrimOp),
}

fn main() {
    println!("Hello, world!");
}
