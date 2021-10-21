//! provide [CountStruct]
//! impl visitor
//! provide [get_span_lines] function
use rustc_ast::ast::{
    AnonConst, Arm, AssocItem, AssocTyConstraint, Async, AttrKind, Attribute, Block,
    BlockCheckMode, Const, EnumDef, Expr, ExprKind, FnHeader, FnRetTy, ForeignItem, GenericArg,
    GenericArgs, GenericBound, GenericParam, Generics, GlobalAsm, Item, ItemKind, Label, Lifetime,
    Local, MacCall, MacroDef, Param, Pat, PatKind, Path, PathSegment, PolyTraitRef, Stmt,
    TraitBoundModifier, TraitRef, Ty, TyKind, Unsafe, UseTree, Variant, VariantData, Visibility,
    WherePredicate,
};
use rustc_ast::node_id::NodeId;
use rustc_ast::visit::{
    self, walk_anon_const, walk_arm, walk_assoc_item, walk_assoc_ty_constraint, walk_attribute,
    walk_block, walk_enum_def, walk_expr, walk_fn, walk_fn_ret_ty, walk_foreign_item,
    walk_generic_arg, walk_generic_args, walk_generic_param, walk_generics, walk_global_asm,
    walk_ident, walk_item, walk_label, walk_lifetime, walk_local, walk_mac, walk_param,
    walk_param_bound, walk_pat, walk_path, walk_path_segment, walk_poly_trait_ref, walk_stmt,
    walk_struct_def, walk_trait_ref, walk_ty, walk_use_tree, walk_variant, walk_vis,
    walk_where_predicate, AssocCtxt, FnKind,
};
use rustc_span::{
    source_map::SourceMap,
    symbol::{Ident, Symbol},
    Span,
};
use std::{collections::HashMap, ops::Deref, str};

pub struct CountStruct<'a> {
    pub feature_counts: HashMap<String, usize>,
    pub macro_counts: HashMap<String, usize>,
    pub macro_defs: Vec<String>,
    pub overload_opt: &'a Vec<&'a str>,
    /// The source_map is necessary to go from a `Span` to actual line & column
    /// numbers for closures.
    pub source_map: &'a SourceMap,
    pub comment_lines: i32,
}

pub fn get_span_lines(source_map: &SourceMap, span: &Span) -> i32 {
    let from_line = source_map.lookup_char_pos(span.lo()).line;
    let to_line = source_map.lookup_char_pos(span.hi()).line;
    (1 + to_line - from_line) as i32
}

impl<'a> CountStruct<'a> {
    fn increase_feature_count(&mut self, item: &str) {
        let count = self.feature_counts.entry(String::from(item)).or_insert(0);
        *count += 1;
    }
    fn increase_macro_use_count(&mut self, item: &str) {
        let count = self.macro_counts.entry(String::from(item)).or_insert(0);
        *count += 1;
    }
    fn increase_comment_lines_by(&mut self, i: i32) {
        self.comment_lines = self.comment_lines + i;
    }
}

/// impl visitor
impl<'ast, 'a> visit::Visitor<'ast> for CountStruct<'a> {
    fn visit_name(&mut self, _span: Span, _name: Symbol) {
        // Nothing to do.
    }
    fn visit_ident(&mut self, ident: Ident) {
        walk_ident(self, ident);
    }
    fn visit_foreign_item(&mut self, i: &'ast ForeignItem) {
        walk_foreign_item(self, i)
    }
    fn visit_global_asm(&mut self, ga: &'ast GlobalAsm) {
        walk_global_asm(self, ga)
    }
    fn visit_item(&mut self, i: &'ast Item) {
        match i.kind {
            ItemKind::ExternCrate(..) => self.increase_feature_count("extern crate"),
            ItemKind::Use(..) => self.increase_feature_count("use"),
            ItemKind::Static(..) => self.increase_feature_count("static"),
            ItemKind::Const(..) => {
                self.increase_feature_count("constant");
            }
            ItemKind::Fn(..) => {} //self.increase_feature_count("function"), fu def 一定会进入visit fn
            ItemKind::Mod(ref safety, ..) => {
                self.increase_feature_count("module");
                match safety {
                    Unsafe::Yes(..) => {
                        self.increase_feature_count("unsafe module");
                        self.increase_feature_count("unsafe");
                    }
                    _ => {}
                }
            }
            ItemKind::ForeignMod(ref _mod) => {
                self.increase_feature_count("foreign module");
                match _mod.unsafety {
                    Unsafe::Yes(..) => {
                        self.increase_feature_count("unsafe");
                        self.increase_feature_count("unsafe module");
                    }
                    _ => {}
                }
            }
            ItemKind::GlobalAsm(..) => self.increase_feature_count("global asm"),
            ItemKind::TyAlias(..) => self.increase_feature_count("type alias"),
            ItemKind::Enum(..) => self.increase_feature_count("enum def"),
            ItemKind::Struct(..) => self.increase_feature_count("struct def"),
            ItemKind::Union(..) => self.increase_feature_count("union def"),
            ItemKind::Trait(ref _traitkind) => {
                self.increase_feature_count("trait def");
                match _traitkind.1 {
                    Unsafe::Yes(..) => self.increase_feature_count("unsafe"),
                    _ => {}
                }
                if _traitkind.3.len() == 1 {
                    self.increase_feature_count("trait single inherited");
                } else if _traitkind.3.len() > 1 {
                    self.increase_feature_count("trait multi inherited");
                }
            }
            ItemKind::TraitAlias(..) => self.increase_feature_count("trait alias"),
            ItemKind::Impl(ref impl_kind) => {
                self.increase_feature_count("implement");
                match impl_kind.unsafety {
                    Unsafe::Yes(..) => self.increase_feature_count("unsafe"),
                    _ => {}
                }
            }
            ItemKind::MacCall(..) => {}
            ItemKind::MacroDef(..) => {
                self.macro_defs
                    .push(String::from(i.ident.name.clone().as_str().deref()));
            }
        }

        walk_item(self, i)
    }
    fn visit_local(&mut self, l: &'ast Local) {
        walk_local(self, l)
    }
    fn visit_block(&mut self, b: &'ast Block) {
        match b.rules {
            BlockCheckMode::Unsafe(..) => {
                self.increase_feature_count("unsafe");
            }
            _ => {}
        }
        walk_block(self, b)
    }
    fn visit_stmt(&mut self, s: &'ast Stmt) {
        walk_stmt(self, s)
    }
    fn visit_param(&mut self, param: &'ast Param) {
        walk_param(self, param)
    }
    fn visit_arm(&mut self, a: &'ast Arm) {
        walk_arm(self, a)
    }
    fn visit_pat(&mut self, p: &'ast Pat) {
        match p.kind {
            PatKind::Wild => self.increase_feature_count("wildcard pattern"),
            PatKind::Struct(..) => self.increase_feature_count("struct pattern"),
            PatKind::Tuple(..) => self.increase_feature_count("tuple pattern"),
            PatKind::TupleStruct(..) => self.increase_feature_count("tuple struct pattern"),
            PatKind::Box(..) => self.increase_feature_count("box pattern"),
            PatKind::Ref(..) => self.increase_feature_count("reference pattern"),
            PatKind::Range(..) => self.increase_feature_count("range pattern"),
            PatKind::Slice(..) => self.increase_feature_count("slice pattern"),
            PatKind::MacCall(..) => {}
            _ => (),
        }
        walk_pat(self, p)
    }
    fn visit_anon_const(&mut self, c: &'ast AnonConst) {
        walk_anon_const(self, c)
    }
    fn visit_expr(&mut self, ex: &'ast Expr) {
        match ex.kind {
            ExprKind::Box(..) => self.increase_feature_count("expr box"),
            ExprKind::Array(..) => self.increase_feature_count("array"),
            ExprKind::Call(..) => self.increase_feature_count("function call"),
            ExprKind::MethodCall(..) => self.increase_feature_count("method call"),
            ExprKind::Cast(..) => self.increase_feature_count("cast"),
            ExprKind::Type(..) => self.increase_feature_count("cast"),
            ExprKind::If(ref head, _, _) => {
                match head.kind {
                    ExprKind::Let(..) => self.increase_feature_count("if let"),
                    _ => {}
                }
                self.increase_feature_count("if");
            }
            //focus! rustc do not have IfLet
            //ExprKind::IfLet(..) => self.increase_feature_count("if let"),
            ExprKind::While(ref sub, _, _) => {
                match sub.kind {
                    ExprKind::Let(..) => self.increase_feature_count("while let"),
                    _ => {}
                }
                self.increase_feature_count("while loop");
            }
            //focus! rustc do not have WhileLet
            //ExprKind::WhileLet(..) => self.increase_feature_count("while let"),
            ExprKind::ForLoop(ref p1, ..) => {
                match p1.kind {
                    PatKind::Ident(..) => self.increase_feature_count("for loop iter"),
                    _ => {}
                }
                self.increase_feature_count("for loop")
            }
            ExprKind::Let(..) => {}
            ExprKind::Loop(..) => self.increase_feature_count("condition-less loop"),
            ExprKind::Match(..) => self.increase_feature_count("match"),
            ExprKind::Closure(_, asyncness, ..) => {
                // self.increase_feature_count("closure"); 一定会进入visit_fn
                match asyncness {
                    Async::Yes { .. } => self.increase_feature_count("async"),
                    _ => (),
                }
            }
            ExprKind::Block(..) => self.increase_feature_count("block"),
            //focus! rustc do not have catch
            //ExprKind::Catch(..) => self.increase_feature_count("catch"),
            ExprKind::Try(..) => self.increase_feature_count("try"),
            ExprKind::TryBlock(..) => self.increase_feature_count("try"),
            ExprKind::Tup(..) => self.increase_feature_count("tuple"),
            ExprKind::Field(..) => self.increase_feature_count("struct"),
            //focus! rustc do not have TupField
            //ExprKind::TupField(..) => self.increase_feature_count("tuple"),
            ExprKind::Range(..) => self.increase_feature_count("range"),
            ExprKind::AddrOf(..) => self.increase_feature_count("reference"),
            ExprKind::InlineAsm(..) => self.increase_feature_count("inline asm"),

            ExprKind::Struct(..) => self.increase_feature_count("struct"),
            ExprKind::Paren(..) => self.increase_feature_count("no op"),
            ExprKind::AssignOp(..) => self.increase_feature_count("assign op"),
            ExprKind::Repeat(..) => {
                self.increase_feature_count("repeated element array construction")
            }
            ExprKind::Binary(..) => self.increase_feature_count("binary"),
            ExprKind::Async(..) => self.increase_feature_count("async"),
            ExprKind::Await(..) => self.increase_feature_count("await"),
            ExprKind::Yield(..) => self.increase_feature_count("yield"),
            ExprKind::MacCall(..) => {} //会最终调用visit mac call
            ExprKind::ConstBlock(..) => {}
            ExprKind::Assign(..) => {}
            ExprKind::Index(..) => {}
            ExprKind::Underscore => {}
            ExprKind::Break(..) => {}
            ExprKind::Continue(..) => {}
            ExprKind::Ret(..) => {}
            ExprKind::LlvmInlineAsm(..) => {}
            ExprKind::Lit(..) => {}
            _ => (),
        }
        walk_expr(self, ex)
    }
    fn visit_expr_post(&mut self, _ex: &'ast Expr) {}
    fn visit_ty(&mut self, t: &'ast Ty) {
        match t.kind {
            TyKind::Slice(..) => self.increase_feature_count("slice"),
            TyKind::Array(..) => self.increase_feature_count("array"),
            TyKind::Ptr(..) => self.increase_feature_count("raw pointer"),
            TyKind::Rptr(..) => self.increase_feature_count("reference"),
            TyKind::Tup(..) => self.increase_feature_count("tuple"),
            TyKind::TraitObject(..) => self.increase_feature_count("trait object"),
            TyKind::ImplTrait(..) => self.increase_feature_count("impl trait"),
            TyKind::Paren(..) => self.increase_feature_count("no op"),
            TyKind::Never => self.increase_feature_count("type never"),
            TyKind::Typeof(..) => self.increase_feature_count("typeof"),
            TyKind::MacCall(..) => {}
            TyKind::BareFn(ref barety) => {
                self.increase_feature_count("bare fn");
                match barety.unsafety {
                    Unsafe::Yes(..) => self.increase_feature_count("unsafe"),
                    _ => {}
                }
            }
            TyKind::Path(..) => {}
            TyKind::Infer => {}
            TyKind::ImplicitSelf => {}
            _ => {}
        }
        walk_ty(self, t)
    }
    fn visit_generic_param(&mut self, param: &'ast GenericParam) {
        self.increase_feature_count("generics");
        walk_generic_param(self, param)
    }
    fn visit_generics(&mut self, g: &'ast Generics) {
        walk_generics(self, g)
    }
    fn visit_where_predicate(&mut self, p: &'ast WherePredicate) {
        self.increase_feature_count("where constraint");
        walk_where_predicate(self, p)
    }
    fn visit_fn(&mut self, fk: FnKind<'ast>, s: Span, _: NodeId) {
        match fk {
            FnKind::Fn(..) => self.increase_feature_count("fn def"), //单纯fn call 不会进入这里
            FnKind::Closure(..) => self.increase_feature_count("closure"),
        };

        // Continue walking the rest of the funciton so we pick up any functions
        // or closures defined in its body.
        walk_fn(self, fk, s)
    }        // let mut nl_cnt = 10;
        // for name in macro_defs.iter() {
        //     let len = name.len();
        //     let ocuppied_len = len / 8 * 8 + 8;
        //     let ocuppied_gap = len / 8 + 1;
        //     if nl_cnt < ocuppied_gap {
        //         println!();
        //         nl_cnt = 10 - ocuppied_gap;
        //     } else {
        //         nl_cnt -= ocuppied_gap;
        //     }
        //     print!("{}", name.with_exact_width(ocuppied_len));
        // }
tem, ctxt: AssocCtxt) {
        walk_assoc_item(self, i, ctxt)
    }
    fn visit_trait_ref(&mut self, t: &'ast TraitRef) {
        let v = &t.path.segments;
        let len = v.len();
        if len > 0 {
            let name = v[len - 1].ident.name.as_str().deref().to_owned();
            if self.overload_opt.contains(&name.as_str()) {
                self.increase_feature_count("operator overload");
            }
        }
        self.increase_feature_count("trait reference");
        walk_trait_ref(self, t)
    }
    fn visit_param_bound(&mut self, bounds: &'ast GenericBound) {
        walk_param_bound(self, bounds)
    }
    fn visit_poly_trait_ref(&mut self, t: &'ast PolyTraitRef, m: &'ast TraitBoundModifier) {
        walk_poly_trait_ref(self, t, m)
    }
    fn visit_variant_data(&mut self, s: &'ast VariantData) {
        walk_struct_def(self, s)
    }

    fn visit_enum_def(
        &mut self,
        enum_definition: &'ast EnumDef,
        generics: &'ast Generics,
        item_id: NodeId,
        _: Span,
    ) {
        self.increase_feature_count("enum");
        walk_enum_def(self, enum_definition, generics, item_id)
    }
    fn visit_variant(&mut self, v: &'ast Variant) {
        self.increase_feature_count("variant");
        walk_variant(self, v)
    }
    fn visit_label(&mut self, label: &'ast Label) {
        walk_label(self, label)
    }
    fn visit_lifetime(&mut self, lifetime: &'ast Lifetime) {
        self.increase_feature_count("lifetime"        // let mut nl_cnt = 10;
        // for name in macro_defs.iter() {
        //     let len = name.len();
        //     let ocuppied_len = len / 8 * 8 + 8;
        //     let ocuppied_gap = len / 8 + 1;
        //     if nl_cnt < ocuppied_gap {
        //         println!();
        //         nl_cnt = 10 - ocuppied_gap;
        //     } else {
        //         nl_cnt -= ocuppied_gap;
        //     }
        //     print!("{}", name.with_exact_width(ocuppied_len));
        // }

    fn visit_mac_call(&mut self, mac: &'ast MacCall) {
        let v = mac.path.segments.clone();
        let l = v.len();
        if l > 0 {
            self.increase_macro_use_count(v[l - 1].ident.name.as_str().deref());
        }
        self.increase_feature_count("macro usage");
        walk_mac(self, mac)
    }
    fn visit_mac_def(&mut self, _mac: &'ast MacroDef, _id: NodeId) {
        self.increase_feature_count("macro def");
        // Nothing to do
    }
    fn visit_path(&mut self, path: &'ast Path, _id: NodeId) {
        walk_path(self, path)
    }
    fn visit_use_tree(&mut self, use_tree: &'ast UseTree, id: NodeId, _nested: bool) {
        walk_use_tree(self, use_tree, id)
    }
    fn visit_path_segment(&mut self, path_span: Span, path_segment: &'ast PathSegment) {
        let name = path_segment.ident.name.as_str();
        if name == "Rc" {
            self.increase_feature_count("Rc");
        } else if name == "RefCell" {
            self.increase_feature_count("RefCell");
        } else if name == "Box" {
            self.increase_feature_count("box usage");
        } else if name == "Arc" {
            self.increase_feature_count("Arc");
        } else if name == "Cell" {
            self.increase_feature_count("Cell");
        } else if name == "Result" {
            self.increase_feature_count("Result");
        }
        walk_path_segment(self, path_span, path_segment)
    }
    fn visit_generic_args(&mut self, path_span: Span, generic_args: &'ast GenericArgs) {
        walk_generic_args(self, path_span, generic_args)
    }
    fn visit_generic_arg(&mut self, generic_arg: &'ast GenericArg) {
        match generic_arg {
            GenericArg::Const(..) => self.increase_feature_count("const generic arg"),
            _ => {}
        }
        walk_generic_arg(self, generic_arg)
    }
    fn visit_assoc_ty_constraint(&mut self, constraint: &'ast AssocTyConstraint) {
        self.increase_feature_count("associate type binding");
        walk_assoc_ty_constraint(self, constraint)
    }
    fn visit_attribute(&mut self, attr: &'ast Attribute) {
        match attr.kind {
            AttrKind::Normal(ref attr, _) => {
                let segments = attr.path.segments.clone();
                let l = segments.len();
                if l > 0 {
                    self.increase_macro_use_count(
                        segments[l - 1].ident.name.clone().as_str().deref(),
                    );
                }
            }
            AttrKind::DocComment(..) => {
                let lines = get_span_lines(self.source_map, &attr.span);
                self.increase_comment_lines_by(lines);
            }        // let mut nl_cnt = 10;
        // for name in macro_defs.iter() {
        //     let len = name.len();
        //     let ocuppied_len = len / 8 * 8 + 8;
        //     let ocuppied_gap = len / 8 + 1;
        //     if nl_cnt < ocuppied_gap {
        //         println!();
        //         nl_cnt = 10 - ocuppied_gap;
        //     } else {
        //         nl_cnt -= ocuppied_gap;
        //     }
        //     print!("{}", name.with_exact_width(ocuppied_len));
        // }

    fn visit_fn_ret_ty(&mut self, ret_ty: &'ast FnRetTy) {
        walk_fn_ret_ty(self, ret_ty)
    }
    fn visit_fn_header(&mut self, header: &'ast FnHeader) {
        // Nothing to do
        match header.asyncness {
            Async::Yes { .. } => self.increase_feature_count("async"),
            _ => (),
        }
        match header.constness {
            Const::Yes(..) => self.increase_feature_count("const fn"),
            _ => (),
        }
    }
}
