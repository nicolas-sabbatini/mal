use anyhow::Result;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use thiserror::Error;

#[derive(Error, Debug)]
enum RuntimeError {
    #[error("User interrupted with CTRL-C")]
    Interrupted,
    #[error("End of file")]
    EndOfFile,
    #[error("Unknown error")]
    Unknown,
}

fn main() -> Result<()> {
    rep()
}

fn read(editor: &mut Editor<()>) -> Result<String, RuntimeError> {
    match editor.readline("user> ") {
        Ok(s) => {
            editor.add_history_entry(&s);
            Ok(s)
        }
        Err(ReadlineError::Interrupted) => Err(RuntimeError::Interrupted),
        Err(ReadlineError::Eof) => Err(RuntimeError::EndOfFile),
        Err(_) => Err(RuntimeError::Unknown),
    }
}

fn eval(string: String) -> Result<String, RuntimeError> {
    Ok(string)
}

fn print(string: String) {
    println!("{string}");
}

fn rep() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    let _ = rl.load_history(".mal_history");
    loop {
        match read(&mut rl).and_then(eval) {
            Ok(s) => print(s),
            Err(err) => {
                print(format!("Error: {err}"));
                break;
            }
        }
    }
    rl.save_history(".mal_history")?;
    Ok(())
}
