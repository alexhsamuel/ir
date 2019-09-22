extern crate libc;

use libc::{c_int, pid_t, rusage};
use std::io;
use std::mem::MaybeUninit;
use std::string::String;
use std::vec::Vec;

use crate::environ::Env;

//------------------------------------------------------------------------------

fn build_strv(args: &Vec<String>) -> Vec<*const i8> {
    // Build argv as a NULL-terminated char* array.
    let mut argv: Vec<*const i8>
        = args.iter().map(|a| { a.as_ptr() as *const i8 }).collect();
    argv.push(std::ptr::null());
    argv
}

//------------------------------------------------------------------------------

#[allow(dead_code)]
pub fn execv(exe: &String, args: &Vec<String>) -> io::Result<()> {
    let res = unsafe {
        libc::execv(exe.as_ptr() as *const i8, build_strv(args).as_ptr())
    };
    // execv only returns on failure, with result -1.
    assert!(res == -1);
    Err(io::Error::last_os_error())
}

pub fn execve(exe: &String, args: &Vec<String>, env: Env) -> io::Result<()> {
    // Construct NAME=val strings for env vars.
    let env = env.into_iter().map(|(n, v)| {
        format!("{}={}", n, v)
    }).collect();

    let res = unsafe {
        libc::execve(
            exe.as_ptr() as *const i8,
            build_strv(args).as_ptr(), 
            build_strv(&env).as_ptr())
    };
    // execve only returns on failure, with result -1.
    assert!(res == -1);
    Err(io::Error::last_os_error())
}

pub fn fork() -> io::Result<pid_t> {
    let child_pid = unsafe { libc::fork() };
    assert!(child_pid >= -1);
    match child_pid {
        -1 => Err(io::Error::last_os_error()),
        _ if child_pid >= 0 => Ok(child_pid),
        _ => panic!("fork returned {}", child_pid),
    }
}

pub fn getpid() -> pid_t {
    unsafe { libc::getpid() }
}

pub fn wait4(pid: pid_t, options: c_int) -> io::Result<(pid_t, c_int, rusage)> {
    let mut status: c_int = 0;
    let mut usage = MaybeUninit::<rusage>::uninit();
    unsafe {
        match libc::wait4(pid, &mut status, options, usage.as_mut_ptr()) {
            -1 => Err(io::Error::last_os_error()),
            child_pid => Ok((child_pid, status, usage.assume_init())),
        }
    }
}

