pub fn has_program(prog: &str) -> bool {
    match std::process::Command::new(prog).arg("--version").output() {
        Ok(out) => !out.stdout.is_empty(),
        _ => {
            println!("cargo:warning=Could not find invokable {}!", prog);
            false
        }
    }
}

pub fn proc_output(args: &[&str]) -> String {
    let out = match std::process::Command::new(args[0])
        .args(&args[1..])
        .output()
    {
        Ok(out) => out.stdout,
        _ => vec![],
    };
    String::from_utf8_lossy(&out).to_string().trim().to_string()
}

pub fn use_static_msvcrt() -> bool {
    cfg!(target_feature = "crt-static") || cfg!(feature = "static-msvcrt")
}

pub fn get_taget_darwin_major_version() -> Option<i32> {
    let env = std::env::var("MACOSX_DEPLOYMENT_TARGET");
    let target = std::env::var("TARGET").unwrap();
    let host = std::env::var("HOST").unwrap();
    if let Ok(env) = env {
        let val: i32 = env
            .trim()
            .split('.')
            .next()
            .expect("Couldn't get macos version!")
            .parse()
            .expect("Counldn't get macos version!");
        Some(val + 9)
    } else if target.contains("darwin") && host.contains("darwin") {
        let val = proc_output(&["uname", "-r"])
            .trim()
            .split('.')
            .next()
            .expect("Couldn't get macos version!")
            .parse()
            .expect("Counldn't get macos version!");
        Some(val)
    } else {
        Some(19)
    }
}
