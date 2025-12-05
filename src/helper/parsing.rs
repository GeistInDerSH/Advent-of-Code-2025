pub fn line_to_range_tuple<S, N>(line: S) -> (N, N)
where
    S: AsRef<str>,
    N: num_traits::Num + std::str::FromStr,
    <N as std::str::FromStr>::Err: std::fmt::Debug,
{
    let (lhs, rhs) = line.as_ref().split_once('-').unwrap();
    (lhs.parse().unwrap(), rhs.parse().unwrap())
}
