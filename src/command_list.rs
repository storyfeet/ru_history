use crate::sort;
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

pub fn top_n_commands<'a, I: Iterator<Item = (&'a String, &'a CommandData)>>(
    it: I,
    dir: &str,
    n: usize,
) -> Vec<String> {
    build_command_list(it, dir, n)
        .v
        .into_iter()
        .take(n)
        .map(|a| a.cmd.to_string())
        .collect()
}

fn build_command_list<'a, I: Iterator<Item = (&'a String, &'a CommandData)>>(
    it: I,
    dir: &str,
    wanted: usize,
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

    sort::top_n(
        &mut v,
        wanted,
        &|b, a| match a.dat.recent.cmp(&b.dat.recent) {
            Ordering::Greater => (20 + 2 * a.score).cmp(&b.score),
            Ordering::Less => a.score.cmp(&(20 + 2 * b.score)),
            Ordering::Equal => a.score.cmp(&b.score),
        },
    );
    CommandList { v }
}
