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
    println!(
        "Output of the command: {}",
        String::from_utf8_lossy(&result.stdout)
    );
    Ok(())
}
