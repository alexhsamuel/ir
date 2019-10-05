extern crate exitcode;

// Used for tests.
#[allow(unused_imports)]
#[macro_use] extern crate maplit;

use ir::environ;
use ir::result;
use ir::spec;
use ir::sys;

fn main() {
    let json_path = match std::env::args().skip(1).next() {
        Some(p) => p,
        None => panic!("no file given"),  // FIXME
    };

    let spec = spec::load_spec_file(&json_path).unwrap_or_else(|err| {
        eprintln!("failed to load {}: {}", json_path, err);
        std::process::exit(exitcode::OSFILE);
    });
    eprintln!("spec: {:?}", spec);
    eprintln!("");

    let env = environ::build(std::env::vars(), &spec.env);

    // Build fd managers.
    let fds = spec.fds.iter().map(|fd_spec| {
        ir::fd::create_fd(fd_spec.fd, &fd_spec.spec).unwrap()
    }).collect::<Vec<_>>();

    for &mut fd in fds {
        (*fd).open_in_parent();
    }

    let child_pid = sys::fork().unwrap_or_else(|err| {
        eprintln!("failed to fork: {}", err);
        std::process::exit(exitcode::OSERR);
    });
    if child_pid == 0 {
        // FIXME: Collect errors and send to parent.
        // Child process.

        let exe = &spec.argv[0];
        let err = sys::execve(exe.clone(), spec.argv.clone(), env).unwrap_err();

        // FIXME: Send this back to the parent process.
        eprintln!("failed to exec: {}", err);
    }
    else {
        // Parent process.
        let (wait_pid, status, rusage) = sys::wait4(child_pid, 0).ok().unwrap();
        assert_eq!(wait_pid, child_pid);  // FIXME: Errors.
        let result = result::ProcResult::new(child_pid, status, rusage);

        eprintln!("");
        result::print(&result);
        println!("");
    }

    std::process::exit(exitcode::OK);
}

