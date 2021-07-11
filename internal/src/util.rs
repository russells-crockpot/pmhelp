use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::{convert::TryFrom, iter::Cycle, ops::RangeInclusive};
use proc_macro2::Span;
use syn::{parse_quote, GenericParam, Generics, Ident, Lifetime, LifetimeDef, TypeParam};

pub struct LetterGenerator(Vec<(char, Cycle<RangeInclusive<char>>)>);

impl LetterGenerator {
    fn new() -> Self {
        Self(vec![Self::new_item()])
    }

    fn new_item() -> (char, Cycle<RangeInclusive<char>>) {
        let mut cycle = ('a'..='z').cycle();
        cycle.next();
        ('a', cycle)
    }

    fn get_current_string(&self) -> String {
        self.0.iter().map(|(c, _)| *c).collect()
    }

    fn inc_combo(&mut self, idx: usize) {
        let value = self.0.get_mut(idx).unwrap();
        let next = value.1.next().unwrap();
        value.0 = next;
        if next == 'a' {
            if idx == 0 {
                self.0.push(Self::new_item());
            } else {
                self.inc_combo(idx - 1);
            }
        }
    }
}

impl Iterator for LetterGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let rval = self.get_current_string();
        self.inc_combo(self.0.len() - 1);
        Some(rval)
    }
}

impl Default for LetterGenerator {
    fn default() -> Self {
        Self::new()
    }
}

pub fn create_new_type_generics<const N: usize>(base: &Generics) -> ([Ident; N], Generics) {
    let mut seen = Vec::new();
    let mut insert_at = None;
    for (i, param) in base.params.iter().enumerate() {
        match &param {
            GenericParam::Lifetime(_) => continue,
            GenericParam::Type(TypeParam { ident, .. }) => {
                if insert_at.is_none() {
                    insert_at = Some(i)
                }
                seen.push(ident.to_string())
            }
            _ => {
                if insert_at.is_none() {
                    insert_at = Some(i)
                }
                break;
            }
        }
    }
    let mut new_types = Vec::with_capacity(N);
    let mut type_gen = LetterGenerator::new().map(|l| format!("'{}", l));
    while new_types.len() < N {
        let name = type_gen.next().unwrap();
        if !seen.contains(&name) {
            let ty: TypeParam = parse_quote! { #name };
            new_types.push(ty);
        }
    }
    let insert_at = insert_at.unwrap_or(0);
    let mut generics = base.clone();
    for ty in new_types.iter() {
        generics
            .params
            .insert(insert_at, GenericParam::Type(ty.clone()));
    }
    let type_idents = new_types
        .into_iter()
        .map(|tp| tp.ident)
        .collect::<Vec<Ident>>();
    if let Ok(ty) = <[Ident; N]>::try_from(type_idents) {
        (ty, generics)
    } else {
        unreachable!()
    }
}

/// Creates the desired number of new lifetimes that won't conflict with the provided `Generics`
/// by continuously iterating through different possibilities of single letters (e.g. `'a`, `'b',
/// `'c', etcetera), adding and additional letter if all other lifetimes are exhausted. This then
/// returns an array of the requested lifetimes, as well as a clone of the `Generics` instance with
/// the lifetimes appended to them.
pub fn create_new_lifetimes<const N: usize>(base: &Generics) -> ([Lifetime; N], Generics) {
    let mut seen_lifetimes = Vec::new();
    let mut insert_at = 0;
    for (i, param) in base.params.iter().enumerate() {
        if let GenericParam::Lifetime(lifetime) = &param {
            seen_lifetimes.push(lifetime.lifetime.ident.to_string());
        } else {
            insert_at = i;
            break;
        }
    }
    let mut new_lifetimes = Vec::with_capacity(N);
    let mut lifetime_gen = LetterGenerator::new().map(|l| format!("'{}", l));
    while new_lifetimes.len() < N {
        let lf_name = lifetime_gen.next().unwrap();
        if !seen_lifetimes.contains(&lf_name) {
            let lifetime = Lifetime::new(&lf_name, Span::call_site());
            new_lifetimes.push(lifetime);
        }
    }
    let mut generics = base.clone();
    for lf in new_lifetimes.iter() {
        let param = GenericParam::Lifetime(LifetimeDef::new(lf.clone()));
        generics.params.insert(insert_at, param);
    }
    if let Ok(lifetimes) = <[Lifetime; N]>::try_from(new_lifetimes) {
        (lifetimes, generics)
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lifetime_generator() {
        let mut gen = LetterGenerator::new();
        for c in 'a'..='z' {
            assert_eq!(gen.next().unwrap(), format!("'{}", c));
        }
        for c in 'a'..='z' {
            assert_eq!(gen.next().unwrap(), format!("'a{}", c));
        }
        for c in 'a'..='z' {
            assert_eq!(gen.next().unwrap(), format!("'b{}", c));
        }
    }
}
