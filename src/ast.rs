/// Mini version of Rust AST
/// Omitted or Modified unnecessary fields

/// An item definition.
pub struct Item {
    pub is_pub: bool,
    /// The name of the item.
    /// Original was Ident structure, but simplified.
    pub ident: String,
    pub kind: ItemKind,
}

pub enum ItemKind {
    /// A function declaration (`fn`).
    ///
    /// E.g., `fn foo(bar: usize) -> usize { .. }`.
    Fn(Fn),
    /// A struct definition (`struct`).
    ///
    /// E.g., `struct Foo<A> { x: A }`.
    Struct,
    /// An enum definition (`enum`).
    ///
    /// E.g., `enum Foo<A, B> { C<A>, D<B> }`.
    Enum,
    /// A module declaration (`mod`).
    ///
    /// E.g., `mod foo;` or `mod foo { .. }`.
    /// `unsafe` keyword on modules is accepted syntactically for macro DSLs, but not
    /// semantically by Rust.
    Mod,
    /// A use declaration item (`use`).
    ///
    /// E.g., `use foo;`, `use foo::bar;` or `use foo::bar as FooBar;`.
    Use,
    /// A trait declaration (`trait`).
    ///
    /// E.g., `trait Foo { .. }`, `trait Foo<T> { .. }` or `auto trait Foo {}`.
    Trait,
    /// A type alias (`type`).
    ///
    /// E.g., `type Foo = Bar<u8>;`.
    TyAlias,
}

pub struct Fn {
    pub generics: Generics,
    pub sig: FnSig,
}

/// Represents lifetime, type and const parameters attached to a declaration of
/// a function, enum, trait, etc.
pub struct Generics {
    pub params: Vec<GenericParam>,
    pub where_clause: WhereClause,
}

pub struct GenericParam {
    pub ident: String,
    pub bounds: Vec<GenericBound>,
    pub kind: GenericParamKind,
}

pub enum GenericBound {
    Trait(Path),
}

/// A "Path" is essentially Rust's notion of a name.
///
/// It's represented as a sequence of identifiers,
/// along with a bunch of supporting information.
///
/// E.g., `std::cmp::PartialEq`.
pub struct Path {
    /// The segments in the path: the things separated by `::`.
    /// Global paths begin with `kw::PathRoot`.
    pub segments: Vec<PathSegment>,
}

/// A segment of a path: an identifier, an optional lifetime, and a set of types.
///
/// E.g., `std`, `String` or `Box<T>`.
pub struct PathSegment {
    /// The identifier portion of this path segment.
    pub ident: String,

    /// Type/lifetime parameters attached to this path. They come in
    /// two flavors: `Path<A,B,C>` and `Path(A,B) -> C`.
    /// `None` means that no parameter list is supplied (`Path`),
    /// `Some` means that parameter list is supplied (`Path<X, Y>`)
    /// but it can be empty (`Path<>`).
    /// `P` is used as a size optimization for the common case with no parameters.
    pub args: Option<GenericArgs>,
}

/// The generic arguments and associated item constraints of a path segment.
///
/// E.g., `<A, B>` as in `Foo<A, B>` or `(A, B)` as in `Foo(A, B)`.
pub struct GenericArgs {
    pub args: Vec<GenericArg>,
}

/// Concrete argument in the sequence of generic args.
pub enum GenericArg {
    /// `Bar` in `Foo<Bar>`.
    Type(Ty),
    // `1` in `Foo<1>`.
    // Const(AnonConst),
}

pub struct Ty {
    pub kind: TyKind,
}

/// The various kinds of type recognized by the compiler.
pub enum TyKind {
    /// A path (`module::module::...::Type`), optionally
    /// "qualified", e.g., `<Vec<T> as SomeTrait>::SomeType`.
    ///
    /// Type parameters are stored in the `Path` itself.
    Path(Path),
}

pub enum GenericParamKind {
    Type,
    // Const,
}

/// A where-clause in a definition.
pub struct WhereClause {
    pub predicates: Vec<WherePredicate>,
}

/// A single predicate in a where-clause.
pub enum WherePredicate {
    /// A type bound (e.g., `for<'c> Foo: Send + Clone + 'c`).
    BoundPredicate(WhereBoundPredicate),
}

/// A type bound.
///
/// E.g., `for<'c> Foo: Send + Clone + 'c`.
pub struct WhereBoundPredicate {
    /// Any generics from a `for` binding.
    pub bound_generic_params: Vec<GenericParam>,
    /// The type being bounded.
    /// Original was pointer of type, but now just parse string without mapping, and map at transpile step
    pub bounded_ty: String,
    /// Trait and lifetime bounds (`Clone + Send + 'static`).
    pub bounds: Vec<GenericBound>,
}

/// Represents a function's signature in a trait declaration,
/// trait implementation, or free function.
pub struct FnSig {
    pub header: FnHeader,
    pub decl: FnDecl,
}

/// A function header.
///
/// All the information between the visibility and the name of the function is
/// included in this struct (e.g., `async unsafe fn` or `const extern "C" fn`).
pub struct FnHeader {
    /// Whether this is `unsafe`, or has a default safety.
    pub safety: Safety,
    /// Whether this is `async`, `gen`, or nothing.
    pub coroutine_kind: Option<CoroutineKind>,
    /// The `const` keyword, if any
    pub constness: Const,
    /// The `extern` keyword and corresponding ABI string, if any.
    pub ext: Extern,
}

/// Safety of items.
pub enum Safety {
    /// `unsafe` an item is explicitly marked as `unsafe`.
    Unsafe,
    /// `safe` an item is explicitly marked as `safe`.
    Safe,
    /// Default means no value was provided, it will take a default value given the context in
    /// which is used.
    Default,
}

/// Describes what kind of coroutine markers, if any, a function has.
///
/// Coroutine markers are things that cause the function to generate a coroutine, such as `async`,
/// which makes the function return `impl Future`, or `gen`, which makes the function return `impl
/// Iterator`.
pub enum CoroutineKind {
    /// `async`, which returns an `impl Future`.
    Async,
    /// `gen`, which returns an `impl Iterator`.
    Gen,
    /// `async gen`, which returns an `impl AsyncIterator`.
    AsyncGen,
}

pub enum Const {
    Yes,
    No,
}

/// `extern` qualifier on a function item or function type.
pub enum Extern {
    /// No explicit extern keyword was used.
    ///
    /// E.g. `fn foo() {}`.
    None,
    /// An explicit extern keyword was used, but with implicit ABI.
    ///
    /// E.g. `extern fn foo() {}`.
    ///
    /// This is just `extern "C"` (see `rustc_target::spec::abi::Abi::FALLBACK`).
    Implicit,
    /// An explicit extern keyword was used with an explicit ABI.
    ///
    /// E.g. `extern "C" fn foo() {}`.
    /// Original was StrLit, but it made it simple
    Explicit(String),
}

/// A signature (not the body) of a function declaration.
///
/// E.g., `fn foo(bar: baz)`.
///
/// Please note that it's different from `FnHeader` structure
/// which contains metadata about function safety, asyncness, constness and ABI.
pub struct FnDecl {
    pub inputs: Vec<Param>,
    pub output: FnRetTy,
}

/// A parameter in a function header.
///
/// E.g., `bar: usize` as in `fn foo(bar: usize)`.
pub struct Param {
    pub ty: Ty,
    pub pat: Pat,
}

pub struct Pat {}  // TODO

pub enum FnRetTy {
    /// Returns type is not specified.
    ///
    /// Functions default to `()` and closures default to inference.
    /// Span points to where return type would be inserted.
    Default,
    /// Everything else.
    Ty(Ty),
}