use std::net::Ipv4Addr;

trait AddrSort {
    fn sort(&mut self);
}

impl AddrSort for Vec<Ipv4Addr> {
    fn sort(&mut self) {
       self.sort_by(|a, b| a.octets().cmp(&b.octets()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut v = vec![Ipv4Addr::new(191, 168, 0, 3), Ipv4Addr::new(192, 168, 0, 2), Ipv4Addr::new(192, 168, 0, 1)];
        v.sort();
        assert_eq!(v, vec![Ipv4Addr::new(191, 168, 0, 3), Ipv4Addr::new(192, 168, 0, 1), Ipv4Addr::new(192, 168, 0, 2)]);
    }
}