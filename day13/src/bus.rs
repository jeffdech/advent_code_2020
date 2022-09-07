#[derive(Debug)]
pub struct ProblemStatement {
    pub departure_time: usize,
    pub buses: Vec<usize>,
    pub posns: Vec<usize>
}

impl ProblemStatement {
    pub fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let departure_line = lines.next().unwrap();
        let bus_line = lines.next().unwrap();
        
        ProblemStatement {
            departure_time: departure_line.parse().unwrap(),
            buses: bus_line
                .split(',')
                .filter(|&s| s != "x")
                .map(|x| x.parse().unwrap())
                .collect(),
            posns: bus_line
                .split(',')
                .enumerate()
                .filter(|&(_, s)| s != "x")
                .map(|(n, _)| n)
                .collect()
        }
    }
}

#[derive(Debug)]
pub struct WaitTime {
    pub bus_id: usize,
    pub wait: usize
}