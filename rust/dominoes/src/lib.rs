use itertools::{Itertools, Position};

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let input_len = input.len();

    if input_len == 0 {
        return Some(Vec::new());
    }

    input
        .iter()
        .permutations(input_len)
        .filter_map(|perm| {
            perm.into_iter()
                .with_position()
                .try_fold(
                    (CandSln::default(), CandSln::default()),
                    |(mut cand_fwd, mut cand_rev), domino| {
                        match domino {
                            Position::Middle(domino) => {
                                let res_fwd = cand_fwd.process_middle(*domino);
                                let res_rev = cand_rev.process_middle(*domino);
                                if let (Err(_), Err(_)) = (res_fwd, res_rev) {
                                    return Err(());
                                }
                            }
                            Position::First(domino) => {
                                cand_fwd.start(*domino);
                                cand_rev.start(*domino);
                            }
                            Position::Last(domino) => {
                                let res_fwd = cand_fwd.end(*domino);
                                let res_rev = cand_rev.end(*domino);
                                if let (Err(_), Err(_)) = (res_fwd, res_rev) {
                                    return Err(());
                                }
                            }
                            Position::Only(domino) => {
                                if domino.0 == domino.1 {
                                    cand_fwd.start(*domino);
                                    cand_rev.fail();
                                } else {
                                    return Err(());
                                }
                            }
                        }
                        Ok((cand_fwd, cand_rev))
                    },
                )
                .ok()
        })
        .map(|(cand_fwd, cand_rev)| {
            if let Some(candidate) = cand_fwd.soln {
                candidate
            } else {
                // safe to unwrap since one of the two must be valid
                cand_rev.soln.unwrap()
            }
        })
        .next()
}

#[derive(Debug, Default)]
struct CandSln {
    start: u8,
    soln: Option<Vec<(u8, u8)>>,
}

impl CandSln {
    fn process_middle(&mut self, domino: (u8, u8)) -> Result<(), ()> {
        if let Some(soln) = &mut self.soln {
            let last = soln.get(soln.len() - 1).ok_or(())?;
            if last.1 == domino.0 {
                soln.push(domino);
            } else if last.1 == domino.1 {
                soln.push((domino.1, domino.0));
            } else {
                self.soln = None;
                return Err(());
            }
        } else {
            return Err(());
        }
        Ok(())
    }

    fn start(&mut self, domino: (u8, u8)) {
        self.start = domino.0;
        self.soln = Some(vec![domino]);
    }

    fn end(&mut self, domino: (u8, u8)) -> Result<(), ()> {
        self.process_middle(domino)?;
        if let Some(soln) = &self.soln {
            let last = soln.get(soln.len() - 1).ok_or(())?;
            if last.1 == self.start {
                return Ok(());
            }
        }
        Err(())
    }

    fn fail(&mut self) {
        self.soln = None;
    }
}
