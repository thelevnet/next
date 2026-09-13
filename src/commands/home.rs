use crate::shell::run;

pub fn switch() -> bool {
    run("nh home switch /etc/nixos")
}



