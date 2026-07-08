use mine_data_structs::minecraft::{Library, Os, OsName, Rule};

fn current_os() -> Os {
    Os {
        name: match std::env::consts::OS {
            "linux" => Some(OsName::Linux),
            "windows" => Some(OsName::Windows),
            _ => None,
        },
        arch: None,
    }
}

fn other_os() -> Os {
    let other_name = match std::env::consts::OS {
        "linux" => OsName::Windows,
        _ => OsName::Linux,
    };
    Os { name: Some(other_name), arch: None }
}

fn lib(rules: Option<Box<[Rule]>>) -> Library {
    Library {
        downloads: None,
        name: String::new(),
        rules,
    }
}

fn rule(action: &str, os: Option<Os>) -> Rule {
    Rule {
        action: action.to_string(),
        os,
    }
}

#[test]
fn no_rules() {
    assert!(lib(None).applies());
}

#[test]
fn empty_rules() {
    assert!(lib(Some(Box::new([]))).applies());
}

#[test]
fn allow_current_os() {
    let l = lib(Some(Box::new([rule("allow", Some(current_os()))])));
    assert!(l.applies());
}

#[test]
fn allow_wrong_os() {
    let l = lib(Some(Box::new([rule("allow", Some(other_os()))])));
    assert!(!l.applies());
}

#[test]
fn disallow_current_os() {
    let l = lib(Some(Box::new([rule("disallow", Some(current_os()))])));
    assert!(!l.applies());
}

#[test]
fn disallow_wrong_os() {
    let l = lib(Some(Box::new([rule("disallow", Some(other_os()))])));
    assert!(l.applies());
}

#[test]
fn allow_all_then_disallow_osx() {
    let l = lib(Some(Box::new([
        rule("allow", None),
        rule("disallow", Some(Os { name: Some(OsName::Osx), arch: None })),
    ])));
    // JSON intent: allow everywhere except OSX.
    assert!(l.applies());
}
