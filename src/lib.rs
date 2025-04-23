//! Wrap an `impl Extend` to map or filter elements before extending.

use std::marker::PhantomData;

/// Wraps a type `Extender: Extend<ExtendElement>` and applies a mapping function before passing elements to the `Extender`.
pub struct ExtendMap<
    'extender,
    InputElement,
    ExtendElement,
    Mapper: FnMut(InputElement) -> ExtendElement,
    Extender: Extend<ExtendElement>,
> {
    extender: &'extender mut Extender,
    mapper: Mapper,
    phantom_data: PhantomData<(InputElement, ExtendElement)>,
}

/// Wraps a type `Extender: Extend<ExtendElement>` and applies a filter function before passing elements to the `Extender`.
pub struct ExtendFilter<
    'extender,
    Element,
    Filter: FnMut(&Element) -> bool,
    Extender: Extend<Element>,
> {
    extender: &'extender mut Extender,
    filter: Filter,
    phantom_data: PhantomData<Element>,
}

impl<
        'extender,
        InputElement,
        ExtendElement,
        Mapper: FnMut(InputElement) -> ExtendElement,
        Extender: Extend<ExtendElement>,
    > ExtendMap<'extender, InputElement, ExtendElement, Mapper, Extender>
{
    pub fn new(extender: &'extender mut Extender, mapper: Mapper) -> Self {
        Self {
            extender,
            mapper,
            phantom_data: PhantomData,
        }
    }

    pub fn into_inner(self) -> &'extender mut Extender {
        self.extender
    }
}

impl<'extender, Element, Filter: FnMut(&Element) -> bool, Extender: Extend<Element>>
    ExtendFilter<'extender, Element, Filter, Extender>
{
    pub fn new(extender: &'extender mut Extender, filter: Filter) -> Self {
        Self {
            extender,
            filter,
            phantom_data: PhantomData,
        }
    }

    pub fn into_inner(self) -> &'extender mut Extender {
        self.extender
    }
}

impl<
        InputElement,
        ExtendElement,
        Mapper: FnMut(InputElement) -> ExtendElement,
        Extender: Extend<ExtendElement>,
    > Extend<InputElement> for ExtendMap<'_, InputElement, ExtendElement, Mapper, Extender>
{
    fn extend<T: IntoIterator<Item = InputElement>>(&mut self, iter: T) {
        self.extender.extend(iter.into_iter().map(&mut self.mapper));
    }
}

impl<Element, Filter: FnMut(&Element) -> bool, Extender: Extend<Element>> Extend<Element>
    for ExtendFilter<'_, Element, Filter, Extender>
{
    fn extend<T: IntoIterator<Item = Element>>(&mut self, iter: T) {
        self.extender
            .extend(iter.into_iter().filter(&mut self.filter));
    }
}
