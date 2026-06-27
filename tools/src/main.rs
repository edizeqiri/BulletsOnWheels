use anyhow::{Context, Result};
use quote::ToTokens;
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::Path,
};
use syn::{
    Expr, FnArg, GenericArgument, Item, PathArguments, Type,
    visit::{self, Visit},
};
use walkdir::WalkDir;

#[derive(Debug, Default)]
struct SystemInfo {
    name: String,
    reads_events: BTreeSet<String>,
    writes_events: BTreeSet<String>,
    triggers_events: BTreeSet<String>,
    reads_messages: BTreeSet<String>,
    writes_messages: BTreeSet<String>,
    reads_state: BTreeSet<String>,
    writes_state: BTreeSet<String>,
    next_state: BTreeSet<String>,
    sets_state_variants: BTreeSet<String>,
    runs_in_states: BTreeSet<String>,
}

fn main() -> Result<()> {
    let root = env::args().nth(1).unwrap_or_else(|| "rust/src".into());
    let out_dir = env::args().nth(2).unwrap_or_else(|| "docs".into());
    let systems = scan_project(Path::new(&root))?;

    fs::create_dir_all(&out_dir)?;
    fs::write(format!("{out_dir}/events.d2"), build_events_d2(&systems))?;
    fs::write(
        format!("{out_dir}/messages.d2"),
        build_messages_d2(&systems),
    )?;
    fs::write(format!("{out_dir}/states.d2"), build_states_d2(&systems))?;

    eprintln!("wrote {out_dir}/events.d2, messages.d2, states.d2");
    Ok(())
}

fn scan_project(root: &Path) -> Result<BTreeMap<String, SystemInfo>> {
    let mut all_systems: BTreeMap<String, SystemInfo> = BTreeMap::new();
    let mut run_in_states: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !path.extension().is_some_and(|e| e == "rs") {
            continue;
        }

        let source = fs::read_to_string(path)
            .with_context(|| format!("failed reading {}", path.display()))?;

        let file = syn::parse_file(&source)
            .with_context(|| format!("failed parsing {}", path.display()))?;

        // Pass 1: scan each function for its own signature and body.
        for item in &file.items {
            if let Item::Fn(func) = item {
                let mut info = SystemInfo {
                    name: func.sig.ident.to_string(),
                    ..Default::default()
                };

                for input in &func.sig.inputs {
                    if let FnArg::Typed(arg) = input {
                        analyze_type(&arg.ty, &mut info);
                    }
                }

                let mut trigger_visitor = TriggerVisitor {
                    triggers: &mut info.triggers_events,
                };
                trigger_visitor.visit_block(&func.block);

                let mut set_state_visitor = SetStateVisitor {
                    sets: &mut info.sets_state_variants,
                };
                set_state_visitor.visit_block(&func.block);

                all_systems.insert(info.name.clone(), info);
            }
        }

        // Pass 2: scan entire file for run_if(in_state(...)) to map systems → states.
        let mut run_if_visitor = RunIfStateVisitor::default();
        run_if_visitor.visit_file(&file);
        for (name, states) in run_if_visitor.result {
            run_in_states.entry(name).or_default().extend(states);
        }
    }

    // Merge run_in_states into system entries (create stub entries if needed).
    for (name, states) in run_in_states {
        all_systems
            .entry(name.clone())
            .or_insert_with(|| SystemInfo {
                name: name.clone(),
                ..Default::default()
            })
            .runs_in_states
            .extend(states);
    }

    all_systems.retain(|_, info| is_interesting(info));
    Ok(all_systems)
}

// ── Visitors ──────────────────────────────────────────────────────────────────

struct TriggerVisitor<'a> {
    triggers: &'a mut BTreeSet<String>,
}

impl<'ast> Visit<'ast> for TriggerVisitor<'_> {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if node.method == "trigger" {
            if let Some(first_arg) = node.args.first() {
                if let Some(name) = extract_event_name_from_expr(first_arg) {
                    self.triggers.insert(name);
                }
            }
        }
        visit::visit_expr_method_call(self, node);
    }
}

struct SetStateVisitor<'a> {
    sets: &'a mut BTreeSet<String>,
}

impl<'ast> Visit<'ast> for SetStateVisitor<'_> {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if node.method == "set_state" {
            if let Some(first_arg) = node.args.first() {
                if let Some(variant) = expr_to_state_variant(first_arg) {
                    self.sets.insert(variant);
                }
            }
        }
        visit::visit_expr_method_call(self, node);
    }
}

#[derive(Default)]
struct RunIfStateVisitor {
    // system_name -> set of state variants it runs in
    result: BTreeMap<String, BTreeSet<String>>,
}

impl<'ast> Visit<'ast> for RunIfStateVisitor {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if node.method == "run_if" {
            if let Some(state) = extract_in_state_from_args(&node.args) {
                for sys_name in extract_system_names_from_expr(&node.receiver) {
                    self.result
                        .entry(sys_name)
                        .or_default()
                        .insert(state.clone());
                }
            }
        }
        visit::visit_expr_method_call(self, node);
    }
}

// ── Extraction helpers ─────────────────────────────────────────────────────────

fn extract_event_name_from_expr(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Path(p) => p.path.segments.last().map(|s| s.ident.to_string()),
        Expr::Struct(s) => s.path.segments.last().map(|s| s.ident.to_string()),
        Expr::Call(c) => {
            if let Expr::Path(p) = c.func.as_ref() {
                p.path.segments.last().map(|s| s.ident.to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Extracts `StateType::VARIANT` from a path expression.
/// Returns None for dynamic/complex expressions like `trigger.event().level_id`.
fn expr_to_state_variant(expr: &Expr) -> Option<String> {
    if let Expr::Path(p) = expr {
        let segs: Vec<_> = p
            .path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect();
        if segs.len() >= 2 {
            return Some(segs.join("::"));
        }
    }
    None
}

/// Extracts the state variant from `run_if(in_state(State::VARIANT))` args.
fn extract_in_state_from_args(
    args: &syn::punctuated::Punctuated<Expr, syn::token::Comma>,
) -> Option<String> {
    let first = args.first()?;
    if let Expr::Call(c) = first {
        if let Expr::Path(p) = c.func.as_ref() {
            if p.path.segments.last()?.ident == "in_state" {
                return c.args.first().and_then(expr_to_state_variant);
            }
        }
    }
    None
}

/// Recursively extracts system function names from the receiver of `.run_if(...)`.
/// Handles: plain fn refs, tuples, chained method calls (run_if, chain, etc).
fn extract_system_names_from_expr(expr: &Expr) -> Vec<String> {
    match expr {
        Expr::Path(p) => p
            .path
            .segments
            .last()
            .map(|s| vec![s.ident.to_string()])
            .unwrap_or_default(),
        Expr::Tuple(t) => t
            .elems
            .iter()
            .flat_map(extract_system_names_from_expr)
            .collect(),
        Expr::MethodCall(mc) => extract_system_names_from_expr(&mc.receiver),
        Expr::Paren(p) => extract_system_names_from_expr(&p.expr),
        _ => vec![],
    }
}

// ── Signature analysis ────────────────────────────────────────────────────────

fn analyze_type(ty: &Type, info: &mut SystemInfo) {
    let Type::Path(type_path) = ty else {
        return;
    };

    let Some(seg) = type_path.path.segments.last() else {
        return;
    };

    let wrapper = seg.ident.to_string();
    let inner = first_generic_type(seg);

    match wrapper.as_str() {
        // Bevy events / messages.
        // Bevy 0.17 renamed EventReader/EventWriter style concepts toward MessageReader/MessageWriter.
        // Keeping both makes the tool useful across versions and codebases.
        "EventReader" => {
            if let Some(t) = inner {
                info.reads_events.insert(t);
            }
        }
        "EventWriter" => {
            if let Some(t) = inner {
                info.writes_events.insert(t);
            }
        }
        "MessageReader" => {
            if let Some(t) = inner {
                info.reads_messages.insert(t);
            }
        }
        "MessageWriter" => {
            if let Some(t) = inner {
                info.writes_messages.insert(t);
            }
        }

        // Observers: Bevy uses Trigger<T> (older) and On<T> (newer) — both consume events.
        "Trigger" | "On" => {
            if let Some(t) = inner {
                info.reads_events.insert(t);
            }
        }

        // State reads.
        "State" => {
            if let Some(t) = inner {
                info.reads_state.insert(t);
            }
        }

        // State writes / transitions.
        "NextState" => {
            if let Some(t) = inner {
                info.next_state.insert(t);
            }
        }

        // Resources may contain state-like control data.
        // This is intentionally conservative.
        "Res" => {
            if let Some(t) = inner {
                if looks_like_state(&t) {
                    info.reads_state.insert(t);
                }
            }
        }
        "ResMut" => {
            if let Some(t) = inner {
                if looks_like_state(&t) {
                    info.writes_state.insert(t);
                }
            }
        }

        _ => {}
    }
}

fn first_generic_type(seg: &syn::PathSegment) -> Option<String> {
    let PathArguments::AngleBracketed(args) = &seg.arguments else {
        return None;
    };

    args.args.iter().find_map(|arg| {
        if let GenericArgument::Type(ty) = arg {
            Some(clean_type_name(ty))
        } else {
            None
        }
    })
}

fn clean_type_name(ty: &Type) -> String {
    ty.to_token_stream()
        .to_string()
        .replace(" :: ", "::")
        .replace(" < ", "<")
        .replace(" >", ">")
        .replace(" , ", ", ")
}

fn looks_like_state(name: &str) -> bool {
    name.ends_with("State")
        || name.ends_with("States")
        || name.contains("GameState")
        || name.contains("AppState")
}

fn is_interesting(info: &SystemInfo) -> bool {
    !info.reads_events.is_empty()
        || !info.writes_events.is_empty()
        || !info.triggers_events.is_empty()
        || !info.reads_messages.is_empty()
        || !info.writes_messages.is_empty()
        || !info.reads_state.is_empty()
        || !info.writes_state.is_empty()
        || !info.next_state.is_empty()
        || !info.sets_state_variants.is_empty()
        || !info.runs_in_states.is_empty()
}

// ── D2 builders ───────────────────────────────────────────────────────────────

fn build_events_d2(systems: &BTreeMap<String, SystemInfo>) -> String {
    let mut out = String::from("direction: right\n\n");

    let relevant: Vec<&SystemInfo> = systems
        .values()
        .filter(|s| {
            !s.reads_events.is_empty()
                || !s.writes_events.is_empty()
                || !s.triggers_events.is_empty()
        })
        .collect();

    let events: BTreeSet<String> = relevant
        .iter()
        .flat_map(|s| {
            s.reads_events
                .iter()
                .chain(s.writes_events.iter())
                .chain(s.triggers_events.iter())
        })
        .cloned()
        .collect();

    for e in &events {
        out.push_str(&format!("{}: {{ shape: parallelogram }}\n", d2_id(e)));
    }
    out.push('\n');
    for s in &relevant {
        out.push_str(&format!("{}: {{ shape: rectangle }}\n", d2_id(&s.name)));
    }
    out.push('\n');

    for info in &relevant {
        let sys = d2_id(&info.name);
        for e in &info.reads_events {
            out.push_str(&format!("{} -> {sys}: reads\n", d2_id(e)));
        }
        for e in &info.writes_events {
            out.push_str(&format!("{sys} -> {}: writes\n", d2_id(e)));
        }
        for e in &info.triggers_events {
            out.push_str(&format!("{sys} -> {}: triggers\n", d2_id(e)));
        }
    }
    out
}

fn build_messages_d2(systems: &BTreeMap<String, SystemInfo>) -> String {
    let mut out = String::from("direction: right\n\n");

    let relevant: Vec<&SystemInfo> = systems
        .values()
        .filter(|s| !s.reads_messages.is_empty() || !s.writes_messages.is_empty())
        .collect();

    let messages: BTreeSet<String> = relevant
        .iter()
        .flat_map(|s| s.reads_messages.iter().chain(s.writes_messages.iter()))
        .cloned()
        .collect();

    for m in &messages {
        out.push_str(&format!("{}: {{ shape: page }}\n", d2_id(m)));
    }
    out.push('\n');
    for s in &relevant {
        out.push_str(&format!("{}: {{ shape: rectangle }}\n", d2_id(&s.name)));
    }
    out.push('\n');

    for info in &relevant {
        let sys = d2_id(&info.name);
        for m in &info.reads_messages {
            out.push_str(&format!("{} -> {sys}: reads\n", d2_id(m)));
        }
        for m in &info.writes_messages {
            out.push_str(&format!("{sys} -> {}: writes\n", d2_id(m)));
        }
    }
    out
}

fn build_states_d2(systems: &BTreeMap<String, SystemInfo>) -> String {
    let mut out = String::from("direction: right\n\n");

    let relevant: Vec<&SystemInfo> = systems
        .values()
        .filter(|s| {
            !s.reads_state.is_empty()
                || !s.writes_state.is_empty()
                || !s.next_state.is_empty()
                || !s.sets_state_variants.is_empty()
                || !s.runs_in_states.is_empty()
        })
        .collect();

    // State types from parameter analysis (e.g. State<InGameState>).
    let state_types: BTreeSet<String> = relevant
        .iter()
        .flat_map(|s| {
            s.reads_state
                .iter()
                .chain(s.writes_state.iter())
                .chain(s.next_state.iter())
        })
        .cloned()
        .collect();

    // State variants from body/builder analysis (e.g. InGameState::RUNNING).
    let state_variants: BTreeSet<String> = relevant
        .iter()
        .flat_map(|s| s.sets_state_variants.iter().chain(s.runs_in_states.iter()))
        .cloned()
        .collect();

    for s in &state_types {
        out.push_str(&format!("{}: {{ shape: hexagon }}\n", d2_id(s)));
    }
    for v in &state_variants {
        out.push_str(&format!("{}: {{ shape: hexagon }}\n", d2_id(v)));
    }
    out.push('\n');
    for s in &relevant {
        out.push_str(&format!("{}: {{ shape: rectangle }}\n", d2_id(&s.name)));
    }
    out.push('\n');

    for info in &relevant {
        let sys = d2_id(&info.name);
        for s in &info.reads_state {
            out.push_str(&format!("{} -> {sys}: reads\n", d2_id(s)));
        }
        for s in &info.writes_state {
            out.push_str(&format!("{sys} -> {}: mutates\n", d2_id(s)));
        }
        for s in &info.next_state {
            out.push_str(&format!("{sys} -> {}: transition\n", d2_id(s)));
        }
        for v in &info.sets_state_variants {
            out.push_str(&format!("{sys} -> {}: set_state\n", d2_id(v)));
        }
        for v in &info.runs_in_states {
            out.push_str(&format!("{} -> {sys}: runs_in\n", d2_id(v)));
        }
    }
    out
}

fn d2_id(name: &str) -> String {
    format!("{:?}", name)
}
