use syn::{Attribute, Field, Lit, Meta, NestedMeta};

fn get_attr_val(attr: &Attribute, attr_name: &str) -> Option<String> {
    if attr.path.is_ident(attr_name) {
        if let Some(Meta::List(_meta_list)) = attr.parse_meta().ok() {
            if let Some(NestedMeta::Lit(Lit::Str(_lit_str))) = _meta_list.nested.first() {
                return Some(_lit_str.value());
            }
        }
    }
    None
}
// 获得形如 #[attrname("AttrValue")] 中的 AttrValue 字符串，作为生成方法的名称。
// 如果 attrname 不符合则返回空。
// Note: AttrValue 可能定义多个，但这里只关注第一个，剩余的忽视掉了。
pub(crate) fn the_first_attr(source: &Field, attr_name: &str) -> Option<String> {
    source
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident(attr_name))
        .next()
        .and_then(|tag_attr| get_attr_val(tag_attr, attr_name))
}