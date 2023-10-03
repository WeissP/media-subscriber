macro_rules! fixed_str {
    ($i:ident, $len:expr, $name:expr, $doc:expr) => {
        #[derive(
            Debug,
            Clone,
            serde::Serialize,
            serde::Deserialize,
            schemars::JsonSchema,
        )]
        #[serde(try_from = "String")]
        #[doc = $doc]
        pub struct $i(pub [u8; $len]);

        impl AsRef<str> for $i {
            fn as_ref(&self) -> &str {
                std::str::from_utf8(&self.0)
                    .expect(concat!($name, " is not valid utf-8"))
            }
        }

        impl TryFrom<String> for $i {
            type Error = String;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Ok(Self(value.as_bytes().try_into().map_err(|_| {
                    format!(
                        "{} should have exact {} characters, received: {}",
                        $name, $len, value
                    )
                })?))
            }
        }
    };
}
pub(crate) use fixed_str;
