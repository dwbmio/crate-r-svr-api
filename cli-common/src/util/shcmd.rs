use log::info;

///执行系统命令
/// win下执行ps1
/// sh下执行sh
///
#[cfg(not(target_os = "windows"))]
pub fn run_sh(cmd: &str, args: &Vec<String>) -> (bool, String) {
    use std::process::Command;
    info!("shell cmd:{}", cmd);
    info!("args:{:?}", args);
    let child = Command::new(cmd).args(args).output();
    match child {
        Ok(c) => {
            let ret = String::from_utf8_lossy(&c.stdout).into_owned();
            if c.status.success() {
                return (true, ret);
            }
            println!("{}", String::from_utf8_lossy(&c.stdout).into_owned());
            return (false, String::from_utf8_lossy(&c.stderr).into_owned());
        }
        Err(e) => {
            return (false, e.to_string());
        }
    }
}

#[cfg(target_os = "windows")]
pub fn run_sh(cmd: &str, args: &Vec<String>) -> (bool, String) {
    info!("shell cmd:{}", cmd);
    info!("args:{:?}", args);
    let exec_src = format!("{base} {args}", base = cmd, args = args.join(" "));
    let (iss, ret) = match powershell_script::run(&exec_src) {
        Ok(v) => (true, v.to_string()),
        Err(e) => (false, e.to_string()),
    };
    (iss, ret)
}

// ///同步执行系统命令(todo)
// pub fn run_sh_async(cmd: &String, arg: &Vec<String>) -> (bool, String) {
//     println!("run cmd bin:{}\n", cmd);
//     println!("args is :{:?}\n", arg);
//     let child = Command::new(cmd)
//         .args(arg)
//         .stdout(Stdio::piped())
//         .spawn()
//         .expect("failed to execute child");

//     let output = child
//         .wait_with_output()
//         .expect("failed to wait on child");
//     let ret = String::from_utf8_lossy(&output.stdout).into_owned();
//     if output.status.success() {
//         return (true, ret);
//     }
//     return (false, String::from_utf8_lossy(&output.stderr).into_owned());
// }
