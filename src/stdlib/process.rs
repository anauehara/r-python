use std::process::{Child, ExitStatus};
use std::io;

pub struct Processo {
    processo: Child,
}

impl Processo {
    /// Aguarda até que o processo termine e retorna seu código de saída
    ///
    /// # Retornos
    /// - `Ok(i32)`: Código de saída do processo (ou -1 se não disponível)
    /// - `Err(io::Error)`: Caso ocorra algum erro ao esperar pelo processo
    ///
    /// # Exemplo
    /// ```
    /// let mut processo = Processo::new("ls", &["-l"]);
    /// match processo.wait() {
    ///     Ok(code) => println!("Processo terminou com código {}", code),
    ///     Err(e) => eprintln!("Erro: {}", e),
    /// }
    /// ```
    pub fn wait(&mut self) -> io::Result<i32> {
        let status: ExitStatus = self.processo.wait()?;
        let exit_code: i32 = status.code().unwrap_or(-1);
        Ok(exit_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_wait_success() -> io::Result<()> {
        // Processo que termina imediatamente com sucesso (código 0)
        let mut processo = Processo {
            processo: Command::new("true").spawn()?
        };
        
        assert_eq!(processo.wait()?, 0);
        Ok(())
    }

    #[test]
    fn test_wait_failure() -> io::Result<()> {
        // Processo que termina imediatamente com erro (código 1)
        let mut processo = Processo {
            processo: Command::new("false").spawn()?
        };
        
        assert_eq!(processo.wait()?, 1);
        Ok(())
    }

    #[test]
    fn test_wait_no_exit_code() {
        // Simula um processo terminado por sinal (sem código de saída)
        let mut processo = Processo {
            processo: Command::new("sleep").arg("5").spawn().unwrap()
        };
        
        processo.processo.kill().unwrap(); // Termina com sinal
        assert_eq!(processo.wait().unwrap(), -1); // Deve retornar -1
    }
}