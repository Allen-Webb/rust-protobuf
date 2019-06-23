// TODO: used by grpc-rust, should move it into separate crate.
#![doc(hidden)]

use std::io::Write;

/// Field visibility.
pub enum Visibility {
    Public,
    Default,
}

pub struct CodeWriter<'a> {
    writer: &'a mut (Write + 'a),
    indent: String,
}

impl<'a> CodeWriter<'a> {
    pub fn new(writer: &'a mut Write) -> CodeWriter<'a> {
        CodeWriter {
            writer: writer,
            indent: "".to_string(),
        }
    }

    pub fn write_line<S: AsRef<str>>(&mut self, line: S) {
        (if line.as_ref().is_empty() {
            self.writer.write_all("\n".as_bytes())
        } else {
            let s: String = [self.indent.as_ref(), line.as_ref(), "\n"].concat();
            self.writer.write_all(s.as_bytes())
        })
        .unwrap();
    }

    pub fn write_generated(&mut self) {
        self.write_line("// This file is generated. Do not edit");
        self.write_generated_common();
    }

    pub fn write_generated_by(&mut self, pkg: &str, version: &str, parser: &str) {
        self.write_line(format!(
            "// This file is generated by {pkg} {version}. Do not edit",
            pkg = pkg,
            version = version
        ));
        self.write_line(format!(
            "// .proto file is parsed by {parser}",
            parser = parser
        ));
        self.write_generated_common();
    }

    fn write_generated_common(&mut self) {
        // https://secure.phabricator.com/T784
        self.write_line("// @generated");

        self.write_line("");
        self.comment("https://github.com/Manishearth/rust-clippy/issues/702");
        self.write_line("#![allow(unknown_lints)]");
        self.write_line("#![allow(clippy)]");
        self.write_line("#![allow(clippy::all)]");
        self.write_line("");
        self.write_line("#![cfg_attr(rustfmt, rustfmt_skip)]");
        self.write_line("");
        self.write_line("#![allow(box_pointers)]");
        self.write_line("#![allow(dead_code)]");
        self.write_line("#![allow(missing_docs)]");
        self.write_line("#![allow(non_camel_case_types)]");
        self.write_line("#![allow(non_snake_case)]");
        self.write_line("#![allow(non_upper_case_globals)]");
        self.write_line("#![allow(trivial_casts)]");
        self.write_line("#![allow(unused_imports)]");
        self.write_line("#![allow(unused_results)]");
    }

    pub fn todo(&mut self, message: &str) {
        self.write_line(format!("panic!(\"TODO: {}\");", message));
    }

    pub fn unimplemented(&mut self) {
        self.write_line(format!("unimplemented!();"));
    }

    pub fn indented<F>(&mut self, cb: F)
    where
        F: FnOnce(&mut CodeWriter),
    {
        cb(&mut CodeWriter {
            writer: self.writer,
            indent: format!("{}    ", self.indent),
        });
    }

    #[allow(dead_code)]
    pub fn commented<F>(&mut self, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        cb(&mut CodeWriter {
            writer: self.writer,
            indent: format!("// {}", self.indent),
        });
    }

    pub fn pub_const(&mut self, name: &str, field_type: &str, init: &str) {
        self.write_line(&format!("pub const {}: {} = {};", name, field_type, init));
    }

    pub fn lazy_static(&mut self, name: &str, ty: &str, protobuf_crate_path: &str) {
        self.write_line(&format!(
            "static {}: {}::rt::Lazy<{}> = {}::rt::Lazy::INIT;",
            name, protobuf_crate_path, ty, protobuf_crate_path,
        ));
    }

    pub fn lazy_static_decl_get<F>(
        &mut self,
        name: &str,
        ty: &str,
        protobuf_crate_path: &str,
        init: F,
    ) where
        F: Fn(&mut CodeWriter),
    {
        self.lazy_static(name, ty, protobuf_crate_path);
        self.write_line(&format!("{}.get(|| {{", name));
        self.indented(|w| init(w));
        self.write_line(&format!("}})"));
    }

    pub fn lazy_static_decl_get_simple(
        &mut self,
        name: &str,
        ty: &str,
        init: &str,
        protobuf_crate_path: &str,
    ) {
        self.lazy_static(name, ty, protobuf_crate_path);
        self.write_line(&format!("{}.get({})", name, init));
    }

    pub fn block<F>(&mut self, first_line: &str, last_line: &str, cb: F)
    where
        F: FnOnce(&mut CodeWriter),
    {
        self.write_line(first_line);
        self.indented(cb);
        self.write_line(last_line);
    }

    pub fn expr_block<F>(&mut self, prefix: &str, cb: F)
    where
        F: FnOnce(&mut CodeWriter),
    {
        self.block(&format!("{} {{", prefix), "}", cb);
    }

    pub fn stmt_block<S: AsRef<str>, F>(&mut self, prefix: S, cb: F)
    where
        F: FnOnce(&mut CodeWriter),
    {
        self.block(&format!("{} {{", prefix.as_ref()), "};", cb);
    }

    pub fn unsafe_expr<F>(&mut self, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        self.expr_block("unsafe", cb);
    }

    pub fn impl_self_block<S: AsRef<str>, F>(&mut self, name: S, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        self.expr_block(&format!("impl {}", name.as_ref()), cb);
    }

    pub fn impl_for_block<S1: AsRef<str>, S2: AsRef<str>, F>(&mut self, tr: S1, ty: S2, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        self.impl_args_for_block(&[], tr.as_ref(), ty.as_ref(), cb);
    }

    pub fn impl_args_for_block<F>(&mut self, args: &[&str], tr: &str, ty: &str, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        let args_str = if args.is_empty() {
            "".to_owned()
        } else {
            format!("<{}>", args.join(", "))
        };
        self.expr_block(&format!("impl{} {} for {}", args_str, tr, ty), cb);
    }

    pub fn unsafe_impl(&mut self, what: &str, for_what: &str) {
        self.write_line(&format!("unsafe impl {} for {} {{}}", what, for_what));
    }

    pub fn pub_struct<S: AsRef<str>, F>(&mut self, name: S, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        self.expr_block(&format!("pub struct {}", name.as_ref()), cb);
    }

    pub fn def_struct<S: AsRef<str>, F>(&mut self, name: S, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        self.expr_block(&format!("struct {}", name.as_ref()), cb);
    }

    pub fn pub_enum<F>(&mut self, name: &str, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        self.expr_block(&format!("pub enum {}", name), cb);
    }

    pub fn pub_trait<F>(&mut self, name: &str, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        self.expr_block(&format!("pub trait {}", name), cb);
    }

    pub fn pub_trait_extend<F>(&mut self, name: &str, extend: &str, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        self.expr_block(&format!("pub trait {} : {}", name, extend), cb);
    }

    pub fn field_entry(&mut self, name: &str, value: &str) {
        self.write_line(&format!("{}: {},", name, value));
    }

    pub fn field_decl(&mut self, name: &str, field_type: &str) {
        self.write_line(&format!("{}: {},", name, field_type));
    }

    pub fn pub_field_decl(&mut self, name: &str, field_type: &str) {
        self.write_line(&format!("pub {}: {},", name, field_type));
    }

    pub fn field_decl_vis(&mut self, vis: Visibility, name: &str, field_type: &str) {
        match vis {
            Visibility::Public => self.pub_field_decl(name, field_type),
            Visibility::Default => self.field_decl(name, field_type),
        }
    }

    pub fn derive(&mut self, derive: &[&str]) {
        let v: Vec<String> = derive.iter().map(|&s| s.to_string()).collect();
        self.write_line(&format!("#[derive({})]", v.join(",")));
    }

    pub fn allow(&mut self, what: &[&str]) {
        let v: Vec<String> = what.iter().map(|&s| s.to_string()).collect();
        self.write_line(&format!("#[allow({})]", v.join(",")));
    }

    pub fn comment(&mut self, comment: &str) {
        if comment.is_empty() {
            self.write_line("//");
        } else {
            self.write_line(&format!("// {}", comment));
        }
    }

    fn documentation(&mut self, comment: &str) {
        if comment.is_empty() {
            self.write_line("///");
        } else {
            self.write_line(&format!("/// {}", comment));
        }
    }

    /// Writes the documentation of the given path.
    ///
    /// Protobuf paths are defined in proto/google/protobuf/descriptor.proto,
    /// in the `SourceCodeInfo` message.
    ///
    /// For example, say we have a file like:
    ///
    /// ```ignore
    /// message Foo {
    ///   optional string foo = 1;
    /// }
    /// ```
    ///
    /// Let's look at just the field definition. We have the following paths:
    ///
    /// ```ignore
    /// path               represents
    /// [ 4, 0, 2, 0 ]     The whole field definition.
    /// [ 4, 0, 2, 0, 4 ]  The label (optional).
    /// [ 4, 0, 2, 0, 5 ]  The type (string).
    /// [ 4, 0, 2, 0, 1 ]  The name (foo).
    /// [ 4, 0, 2, 0, 3 ]  The number (1).
    /// ```
    ///
    /// The `4`s can be obtained using simple introspection:
    ///
    /// ```
    /// use protobuf::descriptor::FileDescriptorProto;
    /// use protobuf::reflect::MessageDescriptor;
    ///
    /// let id = MessageDescriptor::for_type::<FileDescriptorProto>()
    ///     .field_by_name("message_type")
    ///     .expect("`message_type` must exist")
    ///     .proto()
    ///     .get_number();
    ///
    /// assert_eq!(id, 4);
    /// ```
    ///
    /// The first `0` here means this path refers to the first message.
    ///
    /// The `2` then refers to the `field` field on the `DescriptorProto` message.
    ///
    /// Then comes another `0` to refer to the first field of the current message.
    ///
    /// Etc.

    pub fn all_documentation(
        &mut self,
        info: Option<&protobuf::descriptor::SourceCodeInfo>,
        path: &[i32],
    ) {
        let doc = info
            .map(|v| &v.location)
            .and_then(|ls| ls.iter().find(|l| l.path == path))
            .map(|l| l.get_leading_comments());

        let lines = doc
            .iter()
            .map(|doc| doc.lines())
            .flatten()
            .collect::<Vec<_>>();

        // Skip comments with code blocks to avoid rustdoc trying to compile them.
        if !lines.iter().any(|line| line.starts_with("    ")) {
            for doc in &lines {
                self.documentation(doc);
            }
        }
    }

    pub fn fn_def(&mut self, sig: &str) {
        self.write_line(&format!("fn {};", sig));
    }

    pub fn fn_block<F>(&mut self, public: bool, sig: &str, cb: F)
    where
        F: FnOnce(&mut CodeWriter),
    {
        if public {
            self.expr_block(&format!("pub fn {}", sig), cb);
        } else {
            self.expr_block(&format!("fn {}", sig), cb);
        }
    }

    pub fn pub_fn<F>(&mut self, sig: &str, cb: F)
    where
        F: FnOnce(&mut CodeWriter),
    {
        self.fn_block(true, sig, cb);
    }

    pub fn def_fn<F>(&mut self, sig: &str, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        self.fn_block(false, sig, cb);
    }

    pub fn def_mod<F>(&mut self, name: &str, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        self.expr_block(&format!("mod {}", name), cb)
    }

    pub fn pub_mod<F>(&mut self, name: &str, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        self.expr_block(&format!("pub mod {}", name), cb)
    }

    pub fn while_block<S: AsRef<str>, F>(&mut self, cond: S, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        self.expr_block(&format!("while {}", cond.as_ref()), cb);
    }

    // if ... { ... }
    pub fn if_stmt<S: AsRef<str>, F>(&mut self, cond: S, cb: F)
    where
        F: FnOnce(&mut CodeWriter),
    {
        self.expr_block(&format!("if {}", cond.as_ref()), cb);
    }

    // if ... {} else { ... }
    pub fn if_else_stmt<S: AsRef<str>, F>(&mut self, cond: S, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        self.write_line(&format!("if {} {{", cond.as_ref()));
        self.write_line("} else {");
        self.indented(cb);
        self.write_line("}");
    }

    // if let ... = ... { ... }
    pub fn if_let_stmt<F>(&mut self, decl: &str, expr: &str, cb: F)
    where
        F: FnOnce(&mut CodeWriter),
    {
        self.if_stmt(&format!("let {} = {}", decl, expr), cb);
    }

    // if let ... = ... { } else { ... }
    pub fn if_let_else_stmt<F>(&mut self, decl: &str, expr: &str, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        self.if_else_stmt(&format!("let {} = {}", decl, expr), cb);
    }

    pub fn for_stmt<S1: AsRef<str>, S2: AsRef<str>, F>(&mut self, over: S1, varn: S2, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        self.stmt_block(&format!("for {} in {}", varn.as_ref(), over.as_ref()), cb)
    }

    pub fn match_block<S: AsRef<str>, F>(&mut self, value: S, cb: F)
    where
        F: FnOnce(&mut CodeWriter),
    {
        self.stmt_block(&format!("match {}", value.as_ref()), cb);
    }

    pub fn match_expr<S: AsRef<str>, F>(&mut self, value: S, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        self.expr_block(&format!("match {}", value.as_ref()), cb);
    }

    pub fn case_block<S: AsRef<str>, F>(&mut self, cond: S, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        self.block(&format!("{} => {{", cond.as_ref()), "},", cb);
    }

    pub fn case_expr<S1: AsRef<str>, S2: AsRef<str>>(&mut self, cond: S1, body: S2) {
        self.write_line(&format!("{} => {},", cond.as_ref(), body.as_ref()));
    }
}
