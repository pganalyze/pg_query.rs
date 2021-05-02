#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

// Type aliases
pub type bits32 = u32;

// Generated types
include!(concat!(env!("OUT_DIR"), "/ast.rs"));

#[derive(Debug, serde::Deserialize)]
pub struct Value {}
/*
    NodeTag		type;			/* tag appropriately (eg. T_String) */
    union ValUnion
{
    int			ival;		/* machine integer */
    char	   *str;		/* string */
}			val;
} Value
*/
