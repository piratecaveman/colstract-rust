use std::ffi::OsStr;
use std::io;
use std::process::Command;

pub fn run_command<T>(command: &[T]) -> Result<(), io::Error>
where
    T: AsRef<OsStr>,
{
    let program = &command[0];
    let mut negotiator = Command::new(program);
    negotiator.args(&command[1..]);

    let result = negotiator.output()?;
    if !result.stdout.is_empty() {
        println!("Stdout: {}", String::from_utf8_lossy(&result.stdout));
    };

    if !result.stderr.is_empty() {
        eprintln!("Stderr: {}", String::from_utf8_lossy(&result.stderr))
    };
    Ok(())
}
