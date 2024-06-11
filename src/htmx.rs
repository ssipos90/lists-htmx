use hypertext::{Attribute, GlobalAttributes};

#[allow(dead_code, non_upper_case_globals)]
pub trait HtmxAttributes: GlobalAttributes {
    const hx_get: Attribute = Attribute;
    const hx_post: Attribute = Attribute;
    const hx_target: Attribute = Attribute;
}

impl<T: GlobalAttributes> HtmxAttributes for T {}
