use hypertext::{Attribute, GlobalAttributes};

#[allow(dead_code, non_upper_case_globals)]
pub trait HtmxAttributes: GlobalAttributes {
    const hx_get: Attribute = Attribute;
    const hx_post: Attribute = Attribute;
    const hx_put: Attribute = Attribute;
    const hx_patch: Attribute = Attribute;
    const hx_delete: Attribute = Attribute;

    const hx_target: Attribute = Attribute;
    const hx_trigger: Attribute = Attribute;
    const hx_swap: Attribute = Attribute;

    const hx_sync: Attribute = Attribute;
    const hx_select: Attribute = Attribute;
    const hx_confirm: Attribute = Attribute;
    const hx_history_elt: Attribute = Attribute;
    const hx_indicate: Attribute = Attribute;
    const hx_push_url: Attribute = Attribute;
    const hx_prompt: Attribute = Attribute;
}

impl<T: GlobalAttributes> HtmxAttributes for T {}
