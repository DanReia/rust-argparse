use std::ffi::OsString;
use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct Arg{
    pub properties: HashMap<OsString,OsString>
}

impl Arg{
    pub fn new(supplied_name:OsString, arg_type: OsString)->Arg{
        let mut props = HashMap::new();
        props.insert(OsString::from("name"),supplied_name);
        props.insert(OsString::from("type"),arg_type);
        let arg = Arg{properties:props};

        arg
    }
}
