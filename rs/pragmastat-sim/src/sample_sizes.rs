/// Parses sample size strings like `"2..100"`, `"2,3,4,5,10..20,50..100"`.
///
/// Supports:
/// - Individual values: `"5"`, `"10"`
/// - Comma-separated: `"2,3,4,5"`
/// - Ranges (inclusive): `"2..100"`, `"100..2"` (reverse)
/// - Mixed: `"2..50,60,70,80,90,100"`
pub fn parse_sample_sizes(input: &str) -> Vec<usize> {
    let mut result = Vec::new();
    for part in input.split(',') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some((start_str, end_str)) = trimmed.split_once("..") {
            if let (Ok(start), Ok(end)) = (
                start_str.trim().parse::<usize>(),
                end_str.trim().parse::<usize>(),
            ) {
                if start <= end {
                    result.extend(start..=end);
                } else {
                    result.extend((end..=start).rev());
                }
            }
        } else if let Ok(value) = trimmed.parse::<usize>() {
            result.push(value);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_value() {
        assert_eq!(parse_sample_sizes("5"), vec![5]);
    }

    #[test]
    fn comma_separated() {
        assert_eq!(parse_sample_sizes("2,3,4"), vec![2, 3, 4]);
    }

    #[test]
    fn range_ascending() {
        assert_eq!(parse_sample_sizes("2..5"), vec![2, 3, 4, 5]);
    }

    #[test]
    fn range_descending() {
        assert_eq!(parse_sample_sizes("5..2"), vec![5, 4, 3, 2]);
    }

    #[test]
    fn mixed() {
        assert_eq!(parse_sample_sizes("2..4,10,20"), vec![2, 3, 4, 10, 20]);
    }

    #[test]
    fn default_bounds() {
        let sizes = parse_sample_sizes("2..50,60,70,80,90,100");
        assert_eq!(sizes.len(), 54);
        assert_eq!(sizes[0], 2);
        assert_eq!(*sizes.last().unwrap(), 100);
    }
}
