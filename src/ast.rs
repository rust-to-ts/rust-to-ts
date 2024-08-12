pub struct Item {
    pub is_pub: Boolean,
    pub ident: String,
    pub kind: ItemKind,
}

pub enum ItemKind {
    Fn(Fn),
    Struct,
    Enum,
    Mod,
    Use,
    Trait,
    TyAlias,
}

pub struct Fn {
    pub generics: Generics,
    pub sig: FnSig,
}

pub struct Generics {
    pub params: Vec<GenericParam>,
    pub where_clause: Vec<GenericParam>,
}

pub struct GenericParam {
    pub ident: String,
    pub bounds: Vec<GenericBound>,
    pub kind: GenericParamKind,
}

pub enum GenericBound {
    Trait(Path),
}

pub struct Path {
    pub segments: Vec<PathSegment>,
}

pub struct PathSegment {
    pub ident: String,
    pub args: Option<GenericArgs>,
}

pub struct GenericArgs {
    pub args: Vec<GenericArg>,
}

pub enum GenericArg {
    Type(Ty),
    // Const(AnonConst),
}

pub struct Ty {
    pub kind: TyKind,
}

pub enum TyKind {
    Path(Path),
}

pub enum GenericParamKind {
    Type,
    // Const,
}

pub struct FnSig {
    pub header: FnHeader,
    pub decl: FnDecl,
}

pub struct FnHeader {
    pub is_async: boolean,
}

pub struct FnDecl {
    pub inputs: Vec<Param>,
    pub output: FnRetTy,
}

pub struct Param {
    pub ty: Ty,
    pub pat: Pat,
}

pub struct Pat {}  // TODO

pub enum FnRetTy {
    Default,
    Ty(Ty),
}