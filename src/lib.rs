use std::{
    io::{BufRead, BufReader, BufWriter, Error, Write},
    process::{Child, Command, Stdio},
};
pub struct Engine {
    handle: Child,
    stdout: BufReader<std::process::ChildStdout>,
    stdin: BufWriter<std::process::ChildStdin>,
    position: String,
    pub moves: Vec<String>,
}

impl Engine {
    pub fn new(path: &str) -> Result<Engine, Error> {
        let mut handle = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let stdin = BufWriter::new(handle.stdin.take().unwrap());
        let stdout = BufReader::new(handle.stdout.take().unwrap());
        let mut engine = Self {
            stdout,
            stdin,
            handle,
            position: String::new(),
            moves: Vec::new(),
        };
        _ = engine.output("Pikafish");
        Ok(engine)
    }

    pub fn input(&mut self, value: &str) -> Result<(), std::io::Error> {
        writeln!(self.stdin, "{}", value)?;
        self.stdin.flush()
    }

    pub fn output(&mut self, id: &str) -> Vec<String> {
        let mut result = Vec::new();
        loop {
            let mut line = String::new();
            if let Ok(_) = self.stdout.read_line(&mut line) {
                if line.len() > 0 {
                    result.push(line.clone());
                    if line.contains(id) {
                        break;
                    }
                }
                continue;
            };
            break;
        }
        result
    }

    pub fn uci(&mut self) -> String {
        writeln!(self.stdin, "uci").unwrap();
        self.stdin.flush().unwrap();
        self.output("uciok").join("")
    }

    pub fn close(&mut self) -> Result<(), std::io::Error> {
        self.input("stop")?;
        self.handle.kill()?;
        self.handle.wait()?;
        Ok(())
    }

    pub fn new_game(&mut self, threads: usize, hash: usize) -> Result<(), std::io::Error> {
        self.input(&format!("setoption name Threads value {threads}"))?;
        self.input(&format!("setoption name Hash value {hash}"))?;
        self.input("setoption name Clear Hash")?;
        self.input("ucinewgame")?;
        self.input("isready")?;
        _ = self.output("readyok");
        self.input("position startpos")?;
        self.position.push_str("position startpos moves ");
        Ok(())
    }

    pub fn to_fen(&mut self) -> String {
        self.input("d").unwrap();
        let values = self.output("Checkers");
        let value = values.get(values.len() - 3).unwrap();
        value[5..].to_string()
    }

    pub fn move_search(
        &mut self,
        mv: &str,
        depth: usize,
        time: usize,
    ) -> (Option<String>, Option<String>) {
        self.position.push_str(&format!("{} ", mv));
        self.moves.push(mv.to_string());
        self.input(&format!("{}", self.position)).unwrap();
        self.input(&format!("go depth {depth} movetime {time}"))
            .unwrap();
        let result = self.output("bestmove");
        match result.last() {
            Some(value) => {
                let best = Some(value[9..13].to_string());
                let ponder = if value.len() >= 25 {
                    Some(value[21..25].to_string())
                } else {
                    None
                };
                (best, ponder)
            }
            None => (None, None),
        }
    }

    pub fn search(
        &mut self,
        fen: Option<&str>,
        depth: Option<usize>,
        time: Option<usize>,
    ) -> (Option<String>, Option<String>) {
        if let Some(fen) = fen {
            self.input(&format!("position fen {}", fen)).unwrap();
        }
        let mut params: String = String::from("go ");
        if let Some(depth) = depth {
            params.push_str(&format!("depth {} ", depth));
        }
        if let Some(time) = time {
            params.push_str(&format!("movetime {} ", time));
        }
        self.input(&params).unwrap();
        let result = self.output("bestmove");
        match result.last() {
            Some(value) => {
                let best = Some(value[9..13].to_string());
                let ponder = if value.len() >= 25 {
                    Some(value[21..25].to_string())
                } else {
                    None
                };
                (best, ponder)
            }
            None => (None, None),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_search() {
        let bin = "/Users/atopx/opensource/chesscc/Pikafish/src/pikafish";
        let mut engine = super::Engine::new(bin).unwrap();
        println!("{}", engine.uci());
        engine.new_game(6, 32).unwrap();
        let (best, ponter) = engine.search(None, Some(10), None);
        println!("best {:?}", best);
        println!("ponter {:?}", ponter);
        println!("fen {}", engine.to_fen());

        let (best, ponter) = engine.move_search(&best.unwrap(), 20, 2000);
        println!("best {:?}", best);
        println!("ponter {:?}", ponter);
        println!("fen {}", engine.to_fen());
        engine.close().unwrap();
    }
}
