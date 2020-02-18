#[derive(Debug)]
pub struct TspData {
    pub node_coords: Vec<NodeCoord>,
}

#[derive(Debug, PartialEq)]
pub struct NodeCoord {
    pub x: f32,
    pub y: f32,
}

impl NodeCoord {
    fn new(x: f32, y: f32) -> NodeCoord {
        NodeCoord { x, y }
    }
}

impl TspData {
    pub fn new(node_coords: Vec<NodeCoord>) -> TspData {
        TspData { node_coords }
    }

    pub fn generate_distance_matrix(&self) -> Vec<Vec<f32>> {
        let mut distance_matrix: Vec<Vec<f32>> = Vec::new();
        for coord in &self.node_coords {
            let mut distances: Vec<f32> = Vec::new();
            for coord2 in &self.node_coords {
                let distance: f32 =
                    ((coord2.x - coord.x).powi(2) + (coord2.y - coord.y).powi(2)).sqrt() as f32;
                distances.push(distance);
            }
            distance_matrix.push(distances);
        }
        distance_matrix
    }
}

impl std::str::FromStr for TspData {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut node_coords = Vec::<NodeCoord>::new();
        let v: Vec<&str> = s.split("\n").collect();
        for l in &v {
            node_coords.push(parse_node_coord_section_line(l.to_owned().trim())?);
        }
        Ok(TspData::new(node_coords))
    }
}

pub fn read(path: &str) -> Result<TspData, Box<dyn std::error::Error + 'static>> {
    let data: TspData = std::fs::read_to_string(path)?.parse()?;
    Ok(data)
}

fn parse_node_coord_section_line(line: &str) -> Result<NodeCoord, &'static str> {
    let parsed_line: Vec<f32> = line
        .split_whitespace()
        .map(|s| s.parse::<f32>().expect("Node coord must be numerical"))
        .collect();
    if parsed_line.len() < 2 {
        return Err("line too short");
    }
    Ok(NodeCoord::new(
        parsed_line.get(0).unwrap().to_owned(),
        parsed_line.get(1).unwrap().to_owned(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dj38() {
        let data = read(&"instances/dj38.tsp").unwrap();
        let expected_node_coords = vec![
            NodeCoord::new(11003.611100, 42102.500000),
            NodeCoord::new(11108.611100, 42373.888900),
            NodeCoord::new(11133.333300, 42885.833300),
            NodeCoord::new(11155.833300, 42712.500000),
            NodeCoord::new(11183.333300, 42933.333300),
            NodeCoord::new(11297.500000, 42853.333300),
            NodeCoord::new(11310.277800, 42929.444400),
            NodeCoord::new(11416.666700, 42983.333300),
            NodeCoord::new(11423.888900, 43000.277800),
            NodeCoord::new(11438.333300, 42057.222200),
            NodeCoord::new(11461.111100, 43252.777800),
            NodeCoord::new(11485.555600, 43187.222200),
            NodeCoord::new(11503.055600, 42855.277800),
            NodeCoord::new(11511.388900, 42106.388900),
            NodeCoord::new(11522.222200, 42841.944400),
            NodeCoord::new(11569.444400, 43136.666700),
            NodeCoord::new(11583.333300, 43150.000000),
            NodeCoord::new(11595.000000, 43148.055600),
            NodeCoord::new(11600.000000, 43150.000000),
            NodeCoord::new(11690.555600, 42686.666700),
            NodeCoord::new(11715.833300, 41836.111100),
            NodeCoord::new(11751.111100, 42814.444400),
            NodeCoord::new(11770.277800, 42651.944400),
            NodeCoord::new(11785.277800, 42884.444400),
            NodeCoord::new(11822.777800, 42673.611100),
            NodeCoord::new(11846.944400, 42660.555600),
            NodeCoord::new(11963.055600, 43290.555600),
            NodeCoord::new(11973.055600, 43026.111100),
            NodeCoord::new(12058.333300, 42195.555600),
            NodeCoord::new(12149.444400, 42477.500000),
            NodeCoord::new(12286.944400, 43355.555600),
            NodeCoord::new(12300.000000, 42433.333300),
            NodeCoord::new(12355.833300, 43156.388900),
            NodeCoord::new(12363.333300, 43189.166700),
            NodeCoord::new(12372.777800, 42711.388900),
            NodeCoord::new(12386.666700, 43334.722200),
            NodeCoord::new(12421.666700, 42895.555600),
            NodeCoord::new(12645.000000, 42973.333300),
        ];
        assert_eq!(data.node_coords, expected_node_coords);
    }
}
