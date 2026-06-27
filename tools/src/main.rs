use anyhow::{Context, Result};
use quote::ToTokens;
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::Path,
};
use syn::{FnArg, GenericArgument, Item, PathArguments, Type};
use walkdir::WalkDir;

#[derive(Debug, Default)]
struct SystemInfo {
    name: String,
    reads_events: BTreeSet<String>,
    writes_events: BTreeSet<String>,
    reads_messages: BTreeSet<String>,
    writes_messages: BTreeSet<String>,
    reads_state: BTreeSet<String>,
    writes_state: BTreeSet<String>,
    next_state: BTreeSet<String>,
}

fn main() -> Result<()> {
    let root = env::args().nth(1).unwrap_or_else(|| "rust/src".into());
    let out_dir = env::args().nth(2).unwrap_or_else(|| "docs".into());
    let systems = scan_project(Path::new(&root))?;

    fs::create_dir_all(&out_dir)?;
    fs::write(format!("{out_dir}/events.d2"), build_events_d2(&systems))?;
    fs::write(format!("{out_dir}/messages.d2"), build_messages_d2(&systems))?;
    fs::write(format!("{out_dir}/states.d2"), build_states_d2(&systems))?;

    eprintln!("wrote {out_dir}/events.d2, messages.d2, states.d2");
    Ok(())
}

fn scan_project(root: &Path) -> Result<BTreeMap<String, SystemInfo>> {
    let mut systems = BTreeMap::new();

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !path.extension().is_some_and(|e| e == "rs") {
            continue;
        }

        let source = fs::read_to_string(path)
            .with_context(|| format!("failed reading {}", path.display()))?;

        let file = syn::parse_file(&source)
            .with_context(|| format!("failed parsing {}", path.display()))?;

        for item in file.items {
            if let Item::Fn(func) = item {
                let mut info = SystemInfo {
                    name: func.sig.ident.to_string(),
                    ..Default::default()
                };

                for input in func.sig.inputs {
                    if let FnArg::Typed(arg) = input {
                        analyze_type(&arg.ty, &mut info);
                    }
                }

                if is_interesting(&info) {
                    systems.insert(info.name.clone(), info);
                }
            }
        }
    }

    Ok(systems)
}

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
        || !info.reads_messages.is_empty()
        || !info.writes_messages.is_empty()
        || !info.reads_state.is_empty()
        || !info.writes_state.is_empty()
        || !info.next_state.is_empty()
}

fn build_events_d2(systems: &BTreeMap<String, SystemInfo>) -> String {
    let mut out = String::from("direction: right\n\n");

    let relevant: Vec<&SystemInfo> = systems
        .values()
        .filter(|s| !s.reads_events.is_empty() || !s.writes_events.is_empty())
        .collect();

    let events: BTreeSet<String> = relevant
        .iter()
        .flat_map(|s| s.reads_events.iter().chain(s.writes_events.iter()))
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
            !s.reads_state.is_empty() || !s.writes_state.is_empty() || !s.next_state.is_empty()
        })
        .collect();

    let states: BTreeSet<String> = relevant
        .iter()
        .flat_map(|s| {
            s.reads_state
                .iter()
                .chain(s.writes_state.iter())
                .chain(s.next_state.iter())
        })
        .cloned()
        .collect();

    for s in &states {
        out.push_str(&format!("{}: {{ shape: hexagon }}\n", d2_id(s)));
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
    }
    out
}

fn d2_id(name: &str) -> String {
    format!("{:?}", name)
}
