use std::process::{Child, ExitStatus};
use std::io;

pub struct Processo {
    processo: Child,
}

impl Processo {
    #[cfg(windows)]
    pub fn terminate(&mut self) -> io::Result<()> {
        // No Windows, .kill() já usa TerminateProcess, que é o comportamento
        // esperado para terminate() e kill() do Python nessa plataforma.
        self.processo.kill()
    }

    #[cfg(not(windows))]
    pub fn terminate(&mut self) -> io::Result<()> {
        // Em sistemas POSIX (Linux, macOS), precisamos enviar SIGTERM manualmente.
        // O .kill() padrão do Rust enviaria SIGKILL, que é muito agressivo.
        
        // 1. Pega o PID (Process ID) do processo filho.
        let pid = Pid::from_raw(self.processo.id() as i32);

        // 2. Envia o sinal SIGTERM para o PID.
        match signal::kill(pid, Signal::SIGTERM) {
            Ok(_) => Ok(()),
            Err(e) => {
                // 3. Converte o erro da crate 'nix' para um erro padrão 'std::io::Error'.
                Err(io::Error::new(io::ErrorKind::Other, e))
            }
        }
    
    }
    #[cfg(test)]
    mod tests {
        use super::*; // Importa tudo do módulo pai (Processo, etc.)
        use std::thread;
        use std::time::Duration;

        #[test]
        fn test_process_creation_and_wait_success() {
            // Testa o caso mais básico: criar um processo que termina com sucesso.
            // O comando `sleep 0.1` é rápido e deve sair com código 0.
            let mut processo = Processo::new("sleep", &["0.1"]).expect("Falha ao criar processo 'sleep'");
            
            let exit_code = processo.wait().expect("Falha ao esperar pelo processo");
            
            // Assert: Verificamos se o código de saída é 0 (sucesso).
            assert_eq!(exit_code, 0);
        }

        #[test]
        fn test_process_creation_fails_for_invalid_command() {
            // Testa se a criação falha quando o comando não existe.
            let resultado = Processo::new("comando_que_definitivamente_nao_existe_123", &[]);
            
            // Assert: Verificamos que o resultado é um erro.
            assert!(resultado.is_err());
        }

        #[test]
        fn test_terminate_long_running_process() {
            // 1. SETUP: Inicia um processo que demoraria muito para terminar sozinho.
            let mut processo = Processo::new("sleep", &["30"]).expect("Falha ao criar processo 'sleep 30'");
            // Dá um tempinho para o SO realmente iniciar o processo.
            thread::sleep(Duration::from_millis(100));

            // 2. ACTION: Chama o método que queremos testar.
            processo.terminate().expect("Falha ao enviar sinal de terminate");

            // 3. ASSERT: Verifica o resultado.
            let exit_code = processo.wait().expect("Falha ao esperar pelo processo terminado");

            #[cfg(not(windows))]
            {
                // Em Linux/macOS, um processo terminado por sinal não tem um código de saída.
                // Nossa função wait() converte isso para -1. Esta é a verificação correta!
                assert_eq!(exit_code, -1, "Em POSIX, o código de saída de um processo terminado por sinal deve ser -1 (na nossa implementação)");
            }
            #[cfg(windows)]
            {
                // No Windows, TerminateProcess força um código de saída, que geralmente é 1.
                assert_eq!(exit_code, 1, "No Windows, o código de saída de um processo terminado geralmente é 1");
            }
        }

        #[test]
        fn test_kill_long_running_process() {
            // O teste para kill() é quase idêntico ao de terminate(),
            // pois ambos resultam em um encerramento anormal.

            // 1. SETUP
            let mut processo = Processo::new("sleep", &["30"]).expect("Falha ao criar processo 'sleep 30'");
            thread::sleep(Duration::from_millis(100));

            // 2. ACTION
            processo.kill().expect("Falha ao enviar sinal de kill");

            // 3. ASSERT
            let exit_code = processo.wait().expect("Falha ao esperar pelo processo morto");

            #[cfg(not(windows))]
            {
                // SIGKILL também resulta em um código de saída "None", que mapeamos para -1.
                assert_eq!(exit_code, -1, "Em POSIX, o código de saída de um processo morto por sinal deve ser -1");
            }
            #[cfg(windows)]
            {
                // O comportamento é o mesmo de terminate() no Windows.
                assert_eq!(exit_code, 1, "No Windows, o código de saída de um processo morto geralmente é 1");
            }
        }
    } 
    
}