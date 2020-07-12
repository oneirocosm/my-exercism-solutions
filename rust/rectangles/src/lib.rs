use itertools::Itertools;
use std::collections::HashSet;

pub fn count(lines: &[&str]) -> u32 {
    let rect_features = RectFeatures::new(lines);
    let vert_edges = rect_features.vert_edges;
    let horiz_edges = rect_features.horiz_edges;

    rect_features
        .corners
        .iter()
        .permutations(2)
        .filter_map(|pair| RectCandidate::new(*pair[0], *pair[1]))
        .filter(|candidate| candidate.all_edges_valid(&vert_edges, &horiz_edges))
        .count() as u32
}

#[derive(Default, Debug)]
struct RectFeatures {
    corners: HashSet<Point>,
    horiz_edges: HashSet<Point>,
    vert_edges: HashSet<Point>,
}

impl RectFeatures {
    fn new(lines: &[&str]) -> Self {
        lines
            .iter()
            .enumerate()
            .map(|(row_num, &row)| {
                row.chars()
                    .enumerate()
                    .map(move |(col_num, elem)| (Point(row_num, col_num), elem))
            })
            .flatten()
            .fold(Self::default(), |mut features, (point, elem)| {
                match elem {
                    '+' => {
                        features.corners.insert(point);
                        features.horiz_edges.insert(point);
                        features.vert_edges.insert(point);
                    }
                    '-' => {
                        features.horiz_edges.insert(point);
                    }
                    '|' => {
                        features.vert_edges.insert(point);
                    }
                    _ => {}
                }
                features
            })
    }
}

#[derive(Debug, Copy, Clone)]
struct RectCandidate {
    low_low: Point,
    high_high: Point,
}

impl RectCandidate {
    fn new(low_low: Point, high_high: Point) -> Option<Self> {
        if low_low.0 < high_high.0 && low_low.1 < high_high.1 {
            Some(Self { low_low, high_high })
        } else {
            None
        }
    }

    fn all_edges_valid(&self, vert_edges: &HashSet<Point>, horiz_edges: &HashSet<Point>) -> bool {
        self.low_vert_edge_valid(vert_edges)
            && self.high_vert_edge_valid(vert_edges)
            && self.low_horiz_edge_valid(horiz_edges)
            && self.high_horiz_edge_valid(horiz_edges)
    }

    fn low_vert_edge_valid(&self, vert_edges: &HashSet<Point>) -> bool {
        let col_num = self.low_low.1;
        (self.low_low.0..=self.high_high.0)
            .map(|row_num| Point(row_num, col_num))
            .all(|point| vert_edges.contains(&point))
    }

    fn high_vert_edge_valid(&self, vert_edges: &HashSet<Point>) -> bool {
        let col_num = self.high_high.1;
        (self.low_low.0..=self.high_high.0)
            .map(|row_num| Point(row_num, col_num))
            .all(|point| vert_edges.contains(&point))
    }

    fn low_horiz_edge_valid(&self, horiz_edges: &HashSet<Point>) -> bool {
        let row_num = self.low_low.0;
        (self.low_low.1..=self.high_high.1)
            .map(|col_num| Point(row_num, col_num))
            .all(|point| horiz_edges.contains(&point))
    }

    fn high_horiz_edge_valid(&self, horiz_edges: &HashSet<Point>) -> bool {
        let row_num = self.high_high.0;
        (self.low_low.1..=self.high_high.1)
            .map(|col_num| Point(row_num, col_num))
            .all(|point| horiz_edges.contains(&point))
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq)]
struct Point(usize, usize);
