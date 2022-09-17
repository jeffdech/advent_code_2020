use crate::ticket::*;

use std::collections::HashSet;

#[derive(Debug)]
pub struct RuleAssignment<'a>(pub Vec<(&'a str, usize)>);

impl<'a> RuleAssignment<'a> {
    pub fn new(info: &'a TicketInfo) -> Self {
        let valids = info.valid_nearbys();

        let mut assignments = info.rules
            .keys()
            .map(|k| {
                let potentials = (0..valids[0].len())
                    .map(|n| (n, valids.iter().map(|v| v[n]).collect::<Vec<_>>()))
                    .filter(|(_, vals)| vals.iter().all(|v| info.obeys_rule(k, &v)))
                    .map(|(n, _)| n)
                    .collect::<Vec<_>>();

                (k, potentials)
            })
            .collect::<Vec<_>>();

        assignments.sort_by_key(|(k, v)| v.len());

        let mut rem_fields = HashSet::<usize>::from_iter(0..valids[0].len());
        let final_assignment = assignments.iter()
            .map(|(k, vs)| {
                let idx = vs.iter().filter(|v| rem_fields.contains(v)).next().unwrap();

                rem_fields.remove(idx);
                (**k, *idx)
            })
            .collect::<Vec<(&'a str, usize)>>();

        Self(final_assignment)
    }

    pub fn ticket_assignment(&self, nums: &TicketNums) -> Self {
        Self(self.0.iter()
            .map(|(k, n)| (*k, nums[*n]))
            .collect())
    }
}