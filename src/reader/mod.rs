use crate::{solver::Node, Error};
use std::str;

#[derive(Debug)]
pub struct TspData {
    pub nodes: Vec<Node>,
}

impl TspData {
    pub fn new(nodes: Vec<Node>) -> TspData {
        TspData { nodes }
    }
}

impl str::FromStr for TspData {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes: Vec<Node> = vec![];
        for x in s.split('\n') {
            let result = parse_node_coord_section_line(x);
            nodes.push(result?);
        }
        Ok(TspData::new(nodes))
    }
}

pub fn read(path: &str) -> Result<TspData, Error> {
    let file_content = std::fs::read_to_string(path);
    if let Ok(content) = file_content {
        content.parse()
    } else {
        Err(Error::InstanceFileNotFound)
    }
}

fn parse_node_coord_section_line(line: &str) -> Result<Node, Error> {
    let mut splitted = line.split_whitespace();
    let x = splitted.next();
    let y = splitted.next();

    if let (Some(x), Some(y)) = (x, y) {
        let parsed_x = x.parse::<f32>();
        let parsed_y = y.parse::<f32>();

        if let (Ok(x), Ok(y)) = (parsed_x, parsed_y) {
            Ok(Node::new(x, y))
        } else {
            Err(Error::UnparsableCoord)
        }
    } else {
        Err(Error::CoordLineTooShort)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dj38() {
        let data = read(&"instances/dj38.tsp").expect("");
        let expected_node_coords = vec![
            Node::new(11003.611100, 42102.500000),
            Node::new(11108.611100, 42373.888900),
            Node::new(11133.333300, 42885.833300),
            Node::new(11155.833300, 42712.500000),
            Node::new(11183.333300, 42933.333300),
            Node::new(11297.500000, 42853.333300),
            Node::new(11310.277800, 42929.444400),
            Node::new(11416.666700, 42983.333300),
            Node::new(11423.888900, 43000.277800),
            Node::new(11438.333300, 42057.222200),
            Node::new(11461.111100, 43252.777800),
            Node::new(11485.555600, 43187.222200),
            Node::new(11503.055600, 42855.277800),
            Node::new(11511.388900, 42106.388900),
            Node::new(11522.222200, 42841.944400),
            Node::new(11569.444400, 43136.666700),
            Node::new(11583.333300, 43150.000000),
            Node::new(11595.000000, 43148.055600),
            Node::new(11600.000000, 43150.000000),
            Node::new(11690.555600, 42686.666700),
            Node::new(11715.833300, 41836.111100),
            Node::new(11751.111100, 42814.444400),
            Node::new(11770.277800, 42651.944400),
            Node::new(11785.277800, 42884.444400),
            Node::new(11822.777800, 42673.611100),
            Node::new(11846.944400, 42660.555600),
            Node::new(11963.055600, 43290.555600),
            Node::new(11973.055600, 43026.111100),
            Node::new(12058.333300, 42195.555600),
            Node::new(12149.444400, 42477.500000),
            Node::new(12286.944400, 43355.555600),
            Node::new(12300.000000, 42433.333300),
            Node::new(12355.833300, 43156.388900),
            Node::new(12363.333300, 43189.166700),
            Node::new(12372.777800, 42711.388900),
            Node::new(12386.666700, 43334.722200),
            Node::new(12421.666700, 42895.555600),
            Node::new(12645.000000, 42973.333300),
        ];
        assert_eq!(data.nodes, expected_node_coords);
    }

    #[test]
    fn file_not_found() {
        let parsed = read(&"instances/dj39.tsp");
        let error = parsed.err().expect("");
        assert!(matches!(error, Error::InstanceFileNotFound))
    }

    #[test]
    fn coord_line_too_short() {
        let parsed = read(&"instances/dj38_coord_line_too_short.tsp");
        let error = parsed.err().expect("");
        assert!(matches!(error, Error::CoordLineTooShort))
    }

    #[test]
    fn unparsable_coord() {
        let parsed = read(&"instances/dj38_unparsable_coord.tsp");
        let error = parsed.err().expect("");
        assert!(matches!(error, Error::UnparsableCoord))
    }
}
