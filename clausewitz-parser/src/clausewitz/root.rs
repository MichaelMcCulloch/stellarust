use nom::combinator::map;

use super::{bracketed::hash_map, Res, Val};

pub fn root<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
    map(hash_map, Val::Dict)(input)
}
#[cfg(test)]
mod tests {
    use crate::clausewitz::tests::helper::assert_result_ok;

    use super::*;
    #[test]
    fn basics() {
        let text = r###"vers_ion0="Herbert v3.2.2"
            version_control_revision=83287
            date="2200.05.01"
            date="0.05.01"
            float=-0.123939887
            "###;

        let result = root(text);
        assert_result_ok(result);
    }

    #[test]
    fn set_numbers_same_line() {
        let text = r###"set_of_numbers={
                40 41
            }
            "###;
        let result = root(text);
        assert_result_ok(result);
    }
    #[test]
    fn space_not_new_line() {
        let text = r###"modules={
                0=shipyard				1=trading_hub			}
                "###;
        let result = root(text);
        assert_result_ok(result);
    }

    #[test]
    fn intel_numbered_dicts() {
        let text = r###"intel={
                                    {
                                        14 {
                                            intel=0
                                            stale_intel={
                                            }
                                        }
                                    }
                                    {
                                        19 {
                                            intel=0
                                            stale_intel={
                                            }
                                        }
                                    }
                                }
"###;
        let result = root(text);

        assert_result_ok(result);
    }

    #[test]
    fn dict_of_dicts() {
        let text = r###"dict_of_dicts={
                icon={
                    category="human"
                    file="flag_human_9.dds"
                }
                background={
                    category="backgrounds"
                    file="00_solid.dds"
                }
                colors={
                    "blue"
                    "black"
                    "null"
                    "null"
                }
            }"###;
        let result = root(text);
        assert_result_ok(result);
    }

    #[test]
    fn quoted__key__ok() {
        let text = r###""The name Of A Ship"=0
            "###;
        let result = root(text);
        assert_result_ok(result);
    }

    #[test]
    fn empty__set__set() {
        let text = r###"empty_set={}
            "###;
        let result = root(text);
        assert_result_ok(result);
    }

    #[test]
    fn root__set_of_strings__accepted() {
        let text = r###"set_of_strings={
                "Ancient Relics Story Pack"
                "Anniversary Portraits"
                "Apocalypse"
            }
            "###;
        let result = root(text);
        assert_result_ok(result);
    }

    #[test]
    fn array__of__arrays() {
        let text = r###"array_of_arrays={
                0={
                    0="a"
                }
                1={
                    0="one"
                }
                2={
                    0="two"
                }
            }
            "###;
        let result = root(text);
        assert_result_ok(result);
    }

    #[test]
    fn identifier__with__underscore() {
        let text = r###"identifier=identi_fire
            "###;
        let result = root(text);
        assert_result_ok(result);
    }

    #[test]
    fn dict__key_identifier_pairs__ok() {
        let text = r###"dict={
                alpha=a
                beta=b
                cthulhu=ilhjok
            }
            "###;
        let result = root(text);
        assert_result_ok(result);
    }
}