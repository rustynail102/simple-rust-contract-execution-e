#![allow(unused_imports, unused_variables)]

use rustyline::error::ReadlineError;
use rustyline::Editor;

use cfg_if::cfg_if;

use scinterpreter::Compile;
cfg_if! {
    if #[cfg(feature = "jit")] {
        use scinterpreter::Jit as Engine;
    }
    else if #[cfg(feature = "interpreter")] {
        use scinterpreter::Interpreter as Engine;
    }
    else if #[cfg(feature = "vm")]{
        use scinterpreter::vm::bytecode::Interpreter as Engine;
        use scinterpreter::VM;
    }
}

// ANCHOR: repl
fn main() {
    let mut rl = Editor::<()>::new();
    println!("SC Interpreter prompt. Expressions are line evaluated.");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                cfg_if! {
                    if #[cfg(any(feature = "jit", feature = "interpreter"))] {
                        match Engine::from_source(&line) {
                            Ok(result) => println!("{}", result),
                            Err(e) => eprintln!("{}", e),
                        };
                    }
                    else if #[cfg(feature = "vm")] {
                        let byte_code = Engine::from_source(&line);
                        println!("byte code: {:?}", byte_code);
                        let mut vm = VM::new(byte_code);
                        vm.run();
                        println!("{}", vm.pop_last());
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    // ANCHOR_END: repl
}
