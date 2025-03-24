use validator::ValidateEmail;

pub struct AccountEmail(String);

impl AccountEmail {
    pub fn parse(s: String) -> Result<AccountEmail, String> {
        if s.validate_email() {
            Ok(AccountEmail(s.to_string()))
        } else {
            Err(format!("{} is not a valid email", s))
        }
    }
}

impl AsRef<str> for AccountEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AccountEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::AccountEmail;
    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;
    use fake::rand::SeedableRng;
    use fake::rand::rngs::StdRng;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        let result = AccountEmail::parse(email);
        assert!(result.is_err());
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "test".to_string();
        let result = AccountEmail::parse(email);
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@test.com".to_string();
        let result = AccountEmail::parse(email);
        assert!(result.is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let email = SafeEmail().fake_with_rng(&mut rng);

            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        AccountEmail::parse(valid_email.0).is_ok()
    }
}
