use std::io;
use std::process::{Child, ExitStatus};
#[cfg(not(windows))]
use nix::{
    sys::signal::{self, Signal},
    unistd::Pid,
};

pub struct Processo {
    pub processo: Child,
}

impl Processo {

    #[cfg(windows)]
    pub fn terminate(&mut self) -> io::Result<()> {
        self.processo.kill()
    }

    #[cfg(not(windows))]
    pub fn terminate(&mut self) -> io::Result<()> {
        let pid = Pid::from_raw(self.processo.id() as i32);
        match signal::kill(pid, Signal::SIGTERM) {
            Ok(_) => Ok(()),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }

    pub fn kill(&mut self) -> io::Result<()> {
        self.processo.kill()
    }
   
}
mod tests {
    use super::*; 
    use std::process::Command;
    use std::thread;
    use std::time::Duration;

    fn create_long_running_command() -> Command {
        if cfg!(windows) {
            let mut cmd = Command::new("timeout");
            cmd.arg("/T").arg("30");
            cmd
        } else {
            let mut cmd = Command::new("sleep");
            cmd.arg("30");
            cmd
        }
    }

    #[test]
    fn test_terminate_a_running_process() {
        let child = create_long_running_command()
            .spawn()
            .expect("Falha ao iniciar processo para o teste de terminate");

        let mut processo = Processo { processo: child };

        thread::sleep(Duration::from_millis(100));

        processo.terminate().expect("Falha ao chamar terminate");

        let exit_code = processo.wait().expect("Falha ao esperar pelo processo terminado");

        let expected_code = if cfg!(windows) { 1 } else { -1 };
        assert_eq!(exit_code, expected_code, "O código de saída após terminate não foi o esperado.");
    }

    #[test]
    fn test_kill_a_running_process() {
        let child = create_long_running_command()
            .spawn()
            .expect("Falha ao iniciar processo para o teste de kill");


        let mut processo = Processo { processo: child };

        thread::sleep(Duration::from_millis(100));

        processo.kill().expect("Falha ao chamar kill");


        let exit_code = processo.wait().expect("Falha ao esperar pelo processo morto");

        let expected_code = if cfg!(windows) { 1 } else { -1 };
        assert_eq!(exit_code, expected_code, "O código de saída após kill não foi o esperado.");
    }

    #[test]
    fn test_wait_on_a_process_that_finishes_normally() {
        let mut command = if cfg!(windows) {
            let mut cmd = Command::new("timeout");
            cmd.arg("/T").arg("1");
            cmd
        } else {
            let mut cmd = Command::new("sleep");
            cmd.arg("1");
            cmd
        };
        
        let child = command.spawn().expect("Falha ao iniciar processo curto");
        let mut processo = Processo { processo: child };
        
        let exit_code = processo.wait().expect("Falha ao esperar pelo processo");
        assert_eq!(exit_code, 0);
    }
}