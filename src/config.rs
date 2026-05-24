
#[derive(Debug)]
pub struct Configuration {
    shell: String,
    editor: String,
    pager: String,
    agent: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            shell: String::from("/bin/bash"),
            editor: String::from("/usr/bin/emacs -nw"),
            pager: String::from("~/.cargo/bin/bat"),
            agent: String::from("agent"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_bash_as_default_shell() {
        let config = Configuration::default();
        assert_eq!(config.shell, "/bin/bash");
    }

    #[test]
    fn uses_emacs_as_default_editor() {
        let config = Configuration::default();
        assert_eq!(config.editor, "/usr/bin/emacs -nw");
    }

    #[test]
    fn uses_bat_as_default_pager() {
        let config = Configuration::default();
        assert_eq!(config.pager, "~/.cargo/bin/bat");
    }

    #[test]
    fn uses_agent_as_default_agent() {
        let config = Configuration::default();
        assert_eq!(config.agent, "agent");
    }
}