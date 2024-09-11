use quick_xml::se::to_string_with_root;
use serde::Serialize;

pub trait ToXml {
    /// Convert the struct to an XML string.
    fn to_xml(&self) -> String;
}

impl<T> ToXml for T
where
    T: Serialize,
{
    /// Convert the struct to an XML string.
    fn to_xml(&self) -> String {
        let type_name = std::any::type_name::<Self>();
        let struct_name = type_name.split("::").last().unwrap().to_lowercase();
        to_string_with_root(&struct_name, &self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestStruct {
        field1: String,
        field2: i32,
    }

    #[test]
    fn test_to_xml_basic() {
        let test_struct = TestStruct {
            field1: "value1".to_string(),
            field2: 42,
        };
        let xml = test_struct.to_xml();
        let expected_xml = "<teststruct><field1>value1</field1><field2>42</field2></teststruct>";
        assert_eq!(xml, expected_xml);
    }

    #[test]
    fn test_to_xml_empty_struct() {
        #[derive(Serialize)]
        struct EmptyStruct;

        let empty_struct = EmptyStruct;
        let xml = empty_struct.to_xml();
        println!("{}", xml);
        assert_eq!(xml, "<emptystruct/>");
    }

    #[test]
    fn test_to_xml_nested_struct() {
        #[derive(Serialize)]
        struct NestedStruct {
            inner: TestStruct,
        }

        let nested_struct = NestedStruct {
            inner: TestStruct {
                field1: "inner_value".to_string(),
                field2: 100,
            },
        };
        let xml = nested_struct.to_xml();
        assert_eq!(
            xml,
            "<nestedstruct><inner><field1>inner_value</field1><field2>100</field2></inner></nestedstruct>"
        );
    }
}
