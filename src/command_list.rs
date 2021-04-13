use crate::CommandData;
use std::cmp::Ordering;

pub struct CommandRes<'a> {
    cmd: &'a str,
    dat: &'a CommandData,
    score: usize,
}

pub struct CommandList<'a> {
    v: Vec<CommandRes<'a>>,
}

impl<'a> CommandList<'a> {
    pub fn sort_for_cmd(&mut self) {}
}

pub fn build_command_list<'a, I: Iterator<Item = (&'a String, &'a CommandData)>>(
    it: I,
    dir: &str,
) -> CommandList<'a> {
    let mut v: Vec<CommandRes<'a>> = it
        .map(|(cmd, dat)| {
            let mut score = dat.hits;
            if let Some(v) = dat.paths.get(dir) {
                score += v.hits;
            }

            CommandRes { cmd, dat, score }
        })
        .collect();
    v.sort_by(|a, b| match a.dat.recent.cmp(&b.dat.recent) {
        Ordering::Greater => (20 + 2 * a.score).cmp(&b.score),
        Ordering::Less => a.score.cmp(&(20 + 2 * b.score)),
        Ordering::Equal => a.score.cmp(&b.score),
    });
    CommandList { v }
}
