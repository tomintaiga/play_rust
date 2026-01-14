use std::fs::read_to_string;

/// Represent single game level
pub struct Level {
    cells: Vec<char>,
    width: usize,
    height: usize,
}

impl Level {
    // Create new empty level
    pub fn new() -> Level {
        Level {
            cells: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    /// Read level from file
    pub fn from_file(file_name: &str) -> Level {
        let lines = read_lines(file_name);

        Level::from_lines(lines)
    }

    /// Read level from Vec<String>
    fn from_lines(lines: Vec<String>) -> Level {
        let mut result = Level::new();
        // Let's get level size
        result.height = lines.len();
        result.width = lines[0].len();

        for line in lines.iter() {
            for cur in line.chars() {
                result.cells.push(cur);
            }
        }

        result
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let result: String = self
            .cells
            .iter()
            .enumerate()
            .map(|(index, &value)| {
                if (index + 1) % self.width == 0 {
                    format!("{value}\n")
                } else {
                    format!("{value}")
                }
            })
            .collect();

        write!(f, "{result}")
    }
}

/// read_lines will read all lines from text file and return them as `Vec<String>`
fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_from_lines() {
        let lines = vec![
            "######".to_string(),
            "#    #".to_string(),
            "#    #".to_string(),
            "#    #".to_string(),
            "#    #".to_string(),
            "######".to_string(),
        ];

        let level = Level::from_lines(lines);

        assert_eq!(level.width, 6);
        assert_eq!(level.height, 6);
    }

    #[test]
    fn test_load_from_file() {
        let mut file = NamedTempFile::new().expect("Can't create file");

        let data = "######\n#    #\n#    #\n#    #\n#    #\n######";

        writeln!(file, "{data}").expect("Failed to write to temp file");

        let path = file.path().to_str().expect("Invalid path");
        let level = Level::from_file(path);

        assert_eq!(level.width, 6);
        assert_eq!(level.height, 6);
    }

    #[test]
    fn test_display() {
        let lines = vec![
            "######".to_string(),
            "#    #".to_string(),
            "#    #".to_string(),
            "#    #".to_string(),
            "#    #".to_string(),
            "######".to_string(),
        ];

        let level = Level::from_lines(lines);
        let displayed = format!("{level}");

        let expected = "######\n#    #\n#    #\n#    #\n#    #\n######\n";

        assert_eq!(displayed, expected);
    }
}
