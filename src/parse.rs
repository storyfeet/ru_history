use crate::*;
use bogobble::*;

#[derive(Debug)]
pub enum Item {
    Hits(usize),
    Recent(u64),
    Path(String),
    Cmd(String),
}

parser! {(File->Vec<Item>)
    (star(("\n ,".istar(),PItem).last()),("\n ,".istar(),EOI)).first()
}

parser! {(PItem->Item)
    or!(
        ("h",common::UInt).map(|(_,n)|Item::Hits(n)),
        ("r",common::UInt).map(|(_,n)|Item::Recent(n as u64)),
        ("p",Quoted).map(|(_,s)|Item::Path(s)),
        ("c",Quoted).map(|(_,s)|Item::Cmd(s)),
    )
}

parser! {(Quoted->String)
    ('\"',chars_until(or!(
                ("\\n").asv('\n'),
                ("\\\"").asv('"'),
                ("\\\\").asv('\\'),
                Any.one(),
    ),'"')).map(|(_,(b,_))|b)
}

pub fn parse_onto(h: &mut HistoryStore, s: &str) -> anyhow::Result<()> {
    let mut pr = File.parse_s(s).map_err(|e| e.strung())?.into_iter();
    let mut cmd = read_to_command(&mut pr, &mut CommandData::new());

    while let Some(c) = cmd {
        match h.mp.get_mut(&c) {
            Some(v) => cmd = read_to_command(&mut pr, v),
            None => {
                let mut cdat = CommandData::new();
                let new_command = c.to_string();
                cmd = read_to_command(&mut pr, &mut cdat);
                h.mp.insert(new_command, cdat);
            }
        }
    }
    Ok(())
}

pub fn read_to_command<I: Iterator<Item = Item>>(
    i: &mut I,
    cd: &mut CommandData,
) -> Option<String> {
    let mut hits = 0;
    let mut recent = 0;
    while let Some(c) = i.next() {
        match c {
            Item::Cmd(c) => return Some(c),
            Item::Hits(h) => {
                cd.hits += h;
                hits = h;
            }
            Item::Recent(r) => recent = r,
            Item::Path(p) => {
                if recent > cd.recent {
                    cd.recent = recent;
                }
                match cd.paths.get_mut(&p) {
                    Some(hi) => {
                        hi.hits += hits;
                        hi.recent = recent;
                    }
                    None => drop(cd.paths.insert(
                        p,
                        HistoryItem {
                            recent,
                            hits,
                            r_hits: 0,
                        },
                    )),
                }
            }
        }
    }
    None
}
