// traits3.rs
//
// Your task is to implement the Licensed trait for both structures and have
// them return the same information without writing the same function twice.
//
// Consider what you can add to the Licensed trait.
//
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a
// hint.



pub trait Licensed {
    type VersionType: std::fmt::Display;

    fn licensing_info(&self) -> String {
        format!("Licensing information for version {}", self.get_version_number())
    }

    fn get_version_number(&self) -> Self::VersionType;
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {
    type VersionType = String;

    fn get_version_number(&self) -> Self::VersionType {
        format!("{}", self.version_number)
    }
} // Don't edit this line

impl Licensed for OtherSoftware {
    type VersionType = String;

    fn get_version_number(&self) -> Self::VersionType {
        self.version_number.clone()
    }
} // Don't edit this line

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
//        let licensing_info = String::from("Licensing information for version 1");
        let mut licensing_info = String::from("Licensing information for version 1");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        let mut licensing_info = String::from("Licensing information for version v2.0.0");
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
