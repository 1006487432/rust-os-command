use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::io::{self, Write};
use std::os::raw::c_char;
use std::process::{self};
use libc::{fork, wait, WIFEXITED, WEXITSTATUS, SIGINT, SIGQUIT, SIG_DFL, SIG_IGN};

const DFL_PROMPT: &str = "> ";

struct Var {
    value: String,
    global: bool,
}

struct VarLib {
    vars: HashMap<String, Var>,
}

impl VarLib {
    fn new() -> VarLib {
        VarLib {
            vars: HashMap::new(),
        }
    }

    fn store(&mut self, name: &str, val: &str) -> Result<(), &'static str> {
        let var = Var {
            value: val.to_string(),
            global: false,
        };
        self.vars.insert(name.to_string(), var);
        Ok(())
    }

    fn lookup(&self, name: &str) -> Option<&str> {
        self.vars.get(name).map(|var| var.value.as_str())
    }

    fn list(&self) {
        for (key, var) in &self.vars {
            if var.global {
                println!("  * {}={}", key, var.value);
            } else {
                println!("    {}={}", key, var.value);
            }
        }
    }

    fn export(&mut self, name: &str) -> Result<(), &'static str> {
        if let Some(var) = self.vars.get_mut(name) {
            var.global = true;
            Ok(())
        } else {
            self.store(name, "")?;
            self.export(name)
        }
    }

    fn environ_to_table(&mut self, environ: env::Vars) -> Result<(), &'static str> {
        for (key, val) in environ {
            self.store(&key, &val)?;
        }
        Ok(())
    }

    fn table_to_environ(&self) -> Vec<CString> {
        self.vars.iter().filter_map(|(key, var)| {
            if var.global {
                Some(CString::new(format!("{}={}", key, var.value)).unwrap())
            } else {
                None
            }
        }).collect()
    }
}

fn main() {
    let prompt = DFL_PROMPT;
    let mut varlib = VarLib::new();
    setup(&mut varlib);

    loop {
        let cmdline = next_cmd(prompt);
        match cmdline {
            Some(line) => {
                let arglist = splitline(&line);
                if !arglist.is_empty() {
                    process(&arglist, &mut varlib);
                }
            }
            None => break,
        }
    }
}

fn setup(varlib: &mut VarLib) {
    let environ: env::Vars = env::vars();
    varlib.environ_to_table(environ).expect("Failed to initialize environment table");

    unsafe {
        libc::signal(SIGINT, SIG_IGN);
        libc::signal(SIGQUIT, SIG_IGN);
    }
}

fn fatal(s1: &str, s2: &str, n: i32) -> ! {
    eprintln!("Error: {}, {}", s1, s2);
    process::exit(n);
}

fn next_cmd(prompt: &str) -> Option<String> {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => None, // EOF
        Ok(_) => Some(input.trim_end().to_string()),
        Err(_) => None,
    }
}

fn splitline(line: &str) -> Vec<String> {
    line.split_whitespace().map(|s| s.to_string()).collect()
}

fn process(args: &[String], varlib: &mut VarLib) {
    if args.is_empty() {
        return;
    }
    if args[0] == "exit" || args[0] == "quit" {
        std::process::exit(0);
    }
    if is_control_command(&args[0]) {
        do_control_command(args);
    } else if ok_to_execute() {
        if !builtin_command(args, varlib) {
            execute(args);
        }
    }
}

fn is_control_command(cmd: &str) -> bool {
    matches!(cmd, "if" | "then" | "fi")
}

fn do_control_command(args: &[String]) -> i32 {
    match args[0].as_str() {
        "if" => { /* Implement if logic */ 0 },
        "then" => { /* Implement then logic */ 0 },
        "fi" => { /* Implement fi logic */ 0 },
        _ => fatal("internal error processing", &args[0], 2),
    }
}

fn ok_to_execute() -> bool {
    true
}

fn builtin_command(args: &[String], varlib: &mut VarLib) -> bool {
    match args[0].as_str() {
        "set" => {
            varlib.list();
            true
        },
        _ if args[0].contains('=') => assign(&args[0], varlib),
        "export" => {
            if let Some(name) = args.get(1) {
                varlib.export(name).expect("Failed to export variable");
            }
            true
        },
        _ => false,
    }
}

fn assign(arg: &str, varlib: &mut VarLib) -> bool {
    if let Some((name, value)) = arg.split_once('=') {
        varlib.store(name, value).expect("Failed to assign variable");
        true
    } else {
        false
    }
}

fn execute(args: &[String]) -> i32 {
    unsafe {
        let pid = fork();
        if pid == -1 {
            eprintln!("fork error");
            -1
        } else if pid == 0 {
            // Child process
            libc::signal(SIGINT, SIG_DFL);
            libc::signal(SIGQUIT, SIG_DFL);

            let c_args: Vec<CString> = args.iter().map(|arg| CString::new(arg.as_str()).unwrap()).collect();
            let c_args_ptrs: Vec<*const c_char> = c_args.iter().map(|arg| arg.as_ptr()).collect();
            libc::execvp(c_args_ptrs[0], c_args_ptrs.as_ptr() as *const *const c_char);
            eprintln!("cannot execute command");
            process::exit(1);
        } else {
            // Parent process
            let mut status = 0;
            if wait(&mut status) == -1 {
                eprintln!("wait error");
            }
            if WIFEXITED(status) {
                WEXITSTATUS(status)
            } else {
                -1
            }
        }
    }
}
