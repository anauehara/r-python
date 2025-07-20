use std::process::{Child, ChildStdin, ChildStdout, ChildStderr};

/// Representa um processo em execução com acesso a stdin, stdout e stderr.
pub struct PopenProcess {
    pub child: Child,
    pub stdin: Option<ChildStdin>,
    pub stdout: Option<ChildStdout>,
    pub stderr: Option<ChildStderr>,
}

/// Executa um comando e retorna um processo com streams abertos (estilo popen)
pub fn popen_command(
    command: Vec<String>,
    options: RunOptions,
) -> Result<PopenProcess, SubprocessError> {
    if command.is_empty() {
        return Err(SubprocessError::InvalidArguments("Command cannot be empty".to_string()));
    }

    let program = &command[0];
    let args = &command[1..];

    let mut cmd = Command::new(program);
    cmd.args(args);
    cmd.stdin(Stdio::piped());

    if options.capture_output {
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
    }

    match cmd.spawn() {
        Ok(mut child) => {
            let stdin = child.stdin.take();
            let stdout = if options.capture_output { child.stdout.take() } else { None };
            let stderr = if options.capture_output { child.stderr.take() } else { None };

            Ok(PopenProcess {
                child,
                stdin,
                stdout,
                stderr,
            })
        }
        Err(e) => Err(SubprocessError::from_io_error(e, program)),
    }
}


use std::io::{Read, Write};

	#[test]
	fn test_popen_cat_stdin_stdout() {
		// Comando que apenas reflete a entrada
		let mut process = popen_command(
			vec!["cat".to_string()],
			RunOptions { shell: false, capture_output: true }
		).expect("Falha ao iniciar processo");

		let input = "Mensagem via stdin\nOutra linha\n";
    
		// Escreve no stdin do processo
		if let Some(stdin) = process.stdin.as_mut() {
			stdin.write_all(input.as_bytes()).expect("Falha ao escrever no stdin");
		}

		// Fecha stdin para que o processo finalize (cat só sai quando stdin fecha)
		drop(process.stdin.take());

		// Espera a saída do processo
		let output = process.child.wait_with_output().expect("Falha ao esperar processo");

		// Verifica se a saída é igual à entrada
		let stdout = String::from_utf8_lossy(&output.stdout);
		assert_eq!(stdout, input);

		// stderr deve estar vazio
		assert!(output.stderr.is_empty());
		assert_eq!(output.status.code().unwrap_or(-1), 0);
	}

	#[test]
	fn test_popen_error_output() {
		let mut process = popen_command(
			vec!["ls".to_string(), "/naoexiste".to_string()],
			RunOptions { shell: false, capture_output: true }
		).expect("Falha ao iniciar processo");

		let output = process.child.wait_with_output().unwrap();

		assert_ne!(output.status.code().unwrap_or(-1), 0);
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(stderr.contains("No such file") || stderr.contains("não existe"));
	}