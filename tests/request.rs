#[cfg(test)]
mod test {
    use libcne::request::find;
    use libcne::cne::Citizenship;

    #[test]
    fn test_find_elector() {
        let elector = tokio_test::block_on(find(Citizenship::V, "19255544".into())).unwrap();
        assert_eq!(elector.citizenship, Citizenship::V);
        assert_eq!(elector.identity, String::from("19255544"));
        assert_eq!(elector.name, String::from("LEDZE PELIN GARCIA TOVAR"));
        assert_eq!(elector.state, String::from("DTTO. CAPITAL"));
        assert_eq!(elector.town, String::from("CE. BLVNO LIBERTADOR"));
        assert_eq!(elector.parish, String::from("PQ. EL RECREO"));
        assert_eq!(
        elector.voting_center,
            String::from("UNIDAD EDUCATIVA STELLA MATUTINA")
        );
        assert_eq!(
            elector.address, 
        String::from("URBANIZACIÓN AVILA FRENTE AVENIDA COROMOTO. IZQUIERDA TRANSVERSAL SAN JOSE A DOS CUADRAS DE LA CAUCHERA FIRESTONE EDIFICIO")
        );
    }

    #[test]
    fn test_find_fail() {
      assert_eq!(true, tokio_test::block_on(find(Citizenship::V, "80232123".into())).is_err());      
    }
}
