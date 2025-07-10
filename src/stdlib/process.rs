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