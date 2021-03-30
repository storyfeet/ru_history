pub mod parse;
pub mod sort;

pub mod command_list;
use command_list::CommandList;
use std::collections::BTreeMap;
use std::io::Write;
use std::ops::Bound;
use str_tools::traits::*;

pub struct HistoryStore {
    mp: BTreeMap<String, CommandData>,
}

pub struct CommandData {
    paths: BTreeMap<String, HistoryItem>,
    changed: bool,
    recent: u64,
    hits: u64,
}

pub struct HistoryItem {
    changed: bool,
    recent: u64,
    hits: u64,
}

impl HistoryStore {
    pub fn add_cmd(&mut self, cmd: &str, dir: &str, time: u64) {
        match self.mp.get_mut(cmd) {
            Some(cd) => {
                cd.add_dir(dir, time);
            }
            None => {}
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

    fn add_dir(&mut self, dir: &str, time: u64) {
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
