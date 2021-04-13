pub mod parse;
pub mod sort;

pub mod command_list;
use command_list::CommandList;
use std::collections::BTreeMap;
use std::io::Write;
use std::ops::Bound;
use str_tools::traits::*;

#[derive(Debug, PartialEq)]
pub struct HistoryStore {
    mp: BTreeMap<String, CommandData>,
}

#[derive(Debug, PartialEq)]
pub struct CommandData {
    paths: BTreeMap<String, HistoryItem>,
    changed: bool,
    recent: usize,
    hits: usize,
}

#[derive(Debug, PartialEq)]
pub struct HistoryItem {
    changed: bool,
    recent: usize,
    hits: usize,
}

impl HistoryStore {
    pub fn new() -> Self {
        Self {
            mp: BTreeMap::new(),
        }
    }
    pub fn add_cmd(&mut self, cmd: &str, dir: &str, time: usize) {
        match self.mp.get_mut(cmd) {
            Some(cd) => {
                cd.add_dir(dir, time);
                cd.hits += 1;
            }
            None => {
                let mut cd = CommandData::new();
                cd.hits += 1;
                cd.add_dir(dir, time);
                self.mp.insert(cmd.to_string(), cd);
            }
        }
    }

    pub fn write_to<W: Write>(&mut self, w: &mut W, clean: bool) -> std::io::Result<()> {
        for (cmd, dat) in &mut self.mp {
            dat.write_to(w, cmd, clean)?;
        }
        Ok(())
    }

    pub fn complete<'a>(&'a self, pcmd: &str, dir: &str) -> CommandList<'a> {
        if pcmd == "" {
            return command_list::build_command_list((&self.mp).into_iter(), dir);
        }
        //Calculate last valid entry
        let mut c_end = pcmd.to_string();
        let cnext = c_end
            .del_char()
            .and_then(|c| std::char::from_u32((c as u32) + 1))
            .unwrap_or('z');
        c_end.push(cnext);
        command_list::build_command_list(
            self.mp
                .range::<str, _>((Bound::Included(pcmd), Bound::Excluded(c_end.as_str()))),
            dir,
        )
    }
}

impl CommandData {
    fn new() -> Self {
        CommandData {
            paths: BTreeMap::new(),
            changed: false,
            recent: 0,
            hits: 0,
        }
    }

    fn write_to<W: Write>(&mut self, w: &mut W, cmd: &str, clean: bool) -> std::io::Result<()> {
        if !clean && !self.changed {
            return Ok(());
        }
        write!(w, "c{}\n", quoted(cmd))?;
        for (k, v) in &mut self.paths {
            v.write_to(w, k, clean)?;
        }
        self.changed = false;
        Ok(())
    }

    fn add_dir(&mut self, dir: &str, time: usize) {
        self.changed = true;
        match self.paths.get_mut(dir) {
            Some(it) => {
                it.recent = time.max(it.recent);
                it.hits += 1;
                it.changed = true;
            }
            None => {
                self.paths.insert(
                    dir.to_string(),
                    HistoryItem {
                        changed: true,
                        recent: time,
                        hits: 1,
                    },
                );
            }
        }
    }
}

impl HistoryItem {
    fn write_to<W: std::io::Write>(
        &mut self,
        w: &mut W,
        path: &str,
        clean: bool,
    ) -> std::io::Result<()> {
        if !clean && !self.changed {
            return Ok(());
        }
        self.changed = false;
        write!(w, "r{},h{},p{}\n", self.recent, self.hits, quoted(path))
    }
}

fn quoted(s: &str) -> String {
    let mut res = "\"".to_string();
    for c in s.chars() {
        match c {
            '\"' => res.push_str("\\\""),
            '\n' => res.push_str("\\n"),
            _ => res.push(c),
        }
    }
    res.push('\"');
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn multi_test() {
        let mut h_store = HistoryStore::new();
        h_store.add_cmd("do_1", "/home", 10);
        h_store.add_cmd("do_2", "/park", 10);
        h_store.add_cmd("do_1", "/car", 10);
        h_store.add_cmd("do_1", "/home", 10);

        let mut v: Vec<u8> = Vec::new();

        h_store.write_to(&mut v, false).expect("Writing error");

        h_store.add_cmd("do_1", "/home", 12);

        h_store.write_to(&mut v, false).expect("Writing error");

        let s = String::from_utf8(v).unwrap();

        let mut h_load = HistoryStore::new();
        parse::parse_onto(&mut h_load, &s).expect("Parse OK");

        assert_eq!(h_load, h_store);
    }
}
