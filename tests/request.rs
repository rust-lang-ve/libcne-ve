#[cfg(test)]
mod test {
    use libcne_ve::cne::Citizenship;
    use libcne_ve::request::find;

    #[test]
    #[ignore]
    fn test_find_elector() {
        let elector = tokio_test::block_on(find(Citizenship::V, "{{ ID }}".into())).unwrap();
        assert_eq!(elector.citizenship, Citizenship::V);
        assert_eq!(elector.identity, String::from("{{ ID }}"));
        assert_eq!(elector.name, String::from("{{ NAME }}"));
        assert_eq!(elector.state, String::from("{{ STATE }}"));
        assert_eq!(elector.town, String::from("{{ TOWN }}"));
        assert_eq!(elector.parish, String::from("{{ PARISH }}"));
        assert_eq!(
            elector.voting_center,
            String::from("{{ VOTING CENTER }}")
        );
        assert_eq!(
            elector.address,
        String::from("{{ ADDRESS }}")
        );
    }

    #[test]
    #[ignore]
    fn test_find_fail() {
        assert_eq!(
            true,
            tokio_test::block_on(find(Citizenship::V, "{{ ID }}".into())).is_err()
        );
    }
}
