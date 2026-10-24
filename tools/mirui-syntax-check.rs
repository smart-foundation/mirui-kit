//! Run alongside the pinned Mirui 0.51 macro sources by check-mirui-syntax.py.
use super::*;

use quote::ToTokens;
use syn::visit::Visit;
struct UiCheck;
impl<'ast> Visit<'ast> for UiCheck {
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if mac.path.is_ident("ui") {
            let (_, tokens) = strip_bound_captures(mac.tokens.clone()).expect("UI captures");
            if try_parse_fn_call_form(&tokens).is_none() {
                let tokens = crate::layout_reactive::preprocess(tokens).expect("layout bindings");
                let tree: DsTree = syn::parse2(tokens.clone()).unwrap_or_else(|e| {
                    panic!(
                        "real Mirui DSL grammar at {:?}: {e}\n{tokens}",
                        e.span().start()
                    )
                });
                let tree = tree.into_ref();
                let mut rune = MiruiRune::new();
                rune.world_expr = scope_world_expr();
                rune.parent_expr = quote! {__mirui_parent};
                decipher(&tree, &mut rune);
                let output = rune.seal().to_string();
                assert!(!output.contains("compile_error !"), "UI lowering: {output}");
            }
        }
        syn::visit::visit_macro(self, mac);
    }
}
#[test]
fn arena_sources_expand_against_real_051_macros() {
    let root = std::path::Path::new(env!("MIRUI_KIT_ROOT"));
    for file in [
        "crates/kit/src/widgets.rs",
        "crates/kit/src/field.rs",
        "crates/kit/src/selection.rs",
        "crates/fm/src/ui/chrome.rs",
        "crates/fm/src/ui/dialog.rs",
        "crates/fm/src/ui/palette.rs",
        "crates/fm/src/ui/content.rs",
        "crates/fm/src/sync.rs",
        "crates/fm/src/model.rs",
        "crates/fm/src/lib.rs",
        "crates/kit/src/keyboard.rs",
        "crates/kit/src/scrollbar.rs",
        "crates/studio/src/model.rs",
        "crates/studio/src/ui.rs",
        "crates/studio/src/lib.rs",
    ] {
        eprintln!("checking {file}");
        let ast = syn::parse_file(&std::fs::read_to_string(root.join(file)).unwrap()).unwrap();
        for mut item in ast.items.clone() {
            let attrs = match &mut item {
                syn::Item::Struct(s) => core::mem::take(&mut s.attrs),
                syn::Item::Fn(f) => core::mem::take(&mut f.attrs),
                syn::Item::Impl(i) => core::mem::take(&mut i.attrs),
                _ => continue,
            };
            for attr in attrs {
                let name = attr.path().segments.last().unwrap().ident.to_string();
                let arg = match &attr.meta {
                    syn::Meta::List(l) => l.tokens.clone(),
                    _ => proc_macro2::TokenStream::new(),
                };
                let input = item.to_token_stream();
                let result = match name.as_str() {
                    "model" => Some(model_attr::expand(arg, input).unwrap()),
                    "component" => Some(component_attr::expand(arg, input).unwrap()),
                    "compose" => Some(compose_attr::expand(arg, input)),
                    "system" => Some(system_attr::expand(arg, input).unwrap()),
                    "view" => Some(view_attr::expand(arg, input).unwrap()),
                    "event" => Some(event_attr::expand(arg, input).unwrap()),
                    "timer" => Some(timer_attr::expand(arg, input).unwrap()),
                    _ => None,
                };
                if let Some(output) = result {
                    assert!(
                        !output.to_string().contains("compile_error !"),
                        "{file} {name}: {output}"
                    );
                }
            }
        }
        UiCheck.visit_file(&ast);
    }
}
