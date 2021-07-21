use std::ffi::OsStr;
use std::io;
use std::process::Command;

pub fn run_command<T>(command: &[T]) -> Result<(), io::Error>
where
    T: AsRef<OsStr>,
{
    let mut negotiator = Command::new("sh");
    negotiator.args(command);

    let result = negotiator.output()?;
    println!(
        "Output of the command: {}",
        String::from_utf8_lossy(&result.stdout)
    );
    Ok(())
}
