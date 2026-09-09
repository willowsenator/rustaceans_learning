use std::marker::PhantomData;

pub struct Unauthenticated;
pub struct Authenticated;

pub struct SshConnection<Stage = Unauthenticated> {
    stage: PhantomData<Stage>,
}

impl Default for SshConnection<Unauthenticated> {
    fn default() -> Self {
        SshConnection { stage: PhantomData }
    }
}

impl SshConnection<Unauthenticated> {
    pub fn connect(host: &str) -> SshConnection<Unauthenticated> {
        println!("Connected host: {}", host);
        SshConnection { stage: PhantomData }
    }

    pub fn authenticate(self, password: &str) -> SshConnection<Authenticated> {
        println!("Authenticated: {}", password);
        SshConnection { stage: PhantomData }
    }
}

impl SshConnection<Authenticated> {
    pub fn run_command(&self, cmd: &str) -> String {
        cmd.to_string()
    }
}

fn main() {
    let conn = SshConnection::connect("example.html");
    let conn = conn.authenticate("password");
    println!("Command: {}", conn.run_command("ls"));
}
