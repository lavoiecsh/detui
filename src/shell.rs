use bytes::Bytes;
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use std::path::PathBuf;
use std::sync::{Arc, RwLock, mpsc};
use std::thread;
use tui_term::vt100::Parser;

pub struct Shell {
    pub parser: Arc<RwLock<Parser>>,
    tx: mpsc::Sender<Bytes>,
}

impl Shell {
    pub fn new_agent(rows: u16, cols: u16, cwd: &PathBuf) -> Self {
        let mut cmd = CommandBuilder::new("agent");
        cmd.cwd(cwd);
        Self::new(rows, cols, cmd)
    }

    pub fn new_default(rows: u16, cols: u16, cwd: &PathBuf) -> Self {
        let mut cmd = CommandBuilder::new_default_prog();
        cmd.cwd(cwd);
        Self::new(rows, cols, cmd)
    }

    fn new(rows: u16, cols: u16, cmd: CommandBuilder) -> Self {
        let pty_system = NativePtySystem::default();
        let pty_pair = pty_system
            .openpty(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .unwrap();

        thread::spawn(move || {
            let result = pty_pair.slave.spawn_command(cmd);
            if result.is_ok() {
                result.unwrap().wait().unwrap();
            }
            drop(pty_pair.slave);
        });

        let mut reader = pty_pair.master.try_clone_reader().unwrap();
        let parser = Arc::new(RwLock::new(Parser::new(rows, cols, 0)));

        {
            let parser = parser.clone();
            thread::spawn(move || {
                let mut buf = [0u8; 8192];
                let mut processed_buf = Vec::new();
                loop {
                    let size = reader.read(&mut buf).unwrap();
                    if size == 0 {
                        break;
                    }
                    processed_buf.extend_from_slice(&buf[..size]);
                    let mut parser = parser.write().unwrap();
                    parser.process(&processed_buf);
                    processed_buf.clear();
                }
            });
        }

        let (tx, rx) = mpsc::channel::<Bytes>();

        thread::spawn(move || {
            let mut writer = pty_pair.master.take_writer().unwrap();
            while let Ok(bytes) = rx.recv() {
                writer.write_all(&bytes).unwrap();
            }
            drop(pty_pair.master);
        });

        Self { parser, tx }
    }

    pub fn handle_key(&self, code: KeyCode, modifiers: KeyModifiers) {
        let chars = match (code, modifiers) {
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => vec![3],
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => vec![4],
            (KeyCode::Backspace, _) => vec![8],
            (KeyCode::Tab, _) => vec![9],
            (KeyCode::Enter, _) => vec![10],
            (KeyCode::Up, _) => vec![27, 91, 65],
            (KeyCode::Down, _) => vec![27, 91, 66],
            (KeyCode::Right, _) => vec![27, 91, 67],
            (KeyCode::Left, _) => vec![27, 91, 68],
            (KeyCode::End, _) => vec![27, 91, 70],
            (KeyCode::Home, _) => vec![27, 91, 72],
            (KeyCode::BackTab, _) => vec![27, 91, 90],
            (KeyCode::Insert, _) => vec![27, 91, 50, 126],
            (KeyCode::Delete, _) => vec![27, 91, 51, 126],
            (KeyCode::PageUp, _) => vec![27, 91, 53, 126],
            (KeyCode::PageDown, _) => vec![27, 91, 54, 126],
            (KeyCode::Char(input), _) => input.to_string().into_bytes(),
            _ => return,
        };
        self.tx.send(Bytes::from(chars)).unwrap();
    }
}
