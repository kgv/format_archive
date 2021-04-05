//! The [`fmt`](core::fmt) extension

use core::fmt::{
    DebugList, DebugMap, DebugSet, DebugStruct, DebugTuple, Display, Formatter, Result,
};

/// Extension methods for [`Formatter`](core::fmt::Formatter)
pub trait FormatterExt<'a> {
    fn display_list<'b>(&'b mut self) -> DisplayList<'b, 'a>;

    fn display_map<'b>(&'b mut self) -> DisplayMap<'b, 'a>;

    fn display_set<'b>(&'b mut self) -> DisplaySet<'b, 'a>;

    fn display_struct<'b>(&'b mut self, name: &str) -> DisplayStruct<'b, 'a>;

    fn display_tuple<'b>(&'b mut self, name: &str) -> DisplayTuple<'b, 'a>;

    fn display_option<T: Display>(&mut self, option: Option<T>) -> Result {
        if let Some(t) = &option {
            self.display_tuple("Some").field(&t).finish()
        } else {
            self.display_tuple("None").finish()
        }
    }
}

impl<'a> FormatterExt<'a> for Formatter<'a> {
    fn display_list<'b>(&'b mut self) -> DisplayList<'b, 'a> {
        DisplayList(self.debug_list())
    }

    fn display_map<'b>(&'b mut self) -> DisplayMap<'b, 'a> {
        DisplayMap(self.debug_map())
    }

    fn display_set<'b>(&'b mut self) -> DisplaySet<'b, 'a> {
        DisplaySet(self.debug_set())
    }

    fn display_struct<'b>(&'b mut self, name: &str) -> DisplayStruct<'b, 'a> {
        DisplayStruct(self.debug_struct(name))
    }

    fn display_tuple<'b>(&'b mut self, name: &str) -> DisplayTuple<'b, 'a> {
        DisplayTuple(self.debug_tuple(name))
    }
}

/// A struct to help with [`Display`] implementations
///
/// Same as [`DebugList`] except that each entry format is [`Display`].
pub struct DisplayList<'a, 'b>(DebugList<'a, 'b>);

impl DisplayList<'_, '_> {
    pub fn entries<T, I>(&mut self, entries: I) -> &mut Self
    where
        T: Display,
        I: IntoIterator<Item = T>,
    {
        for entry in entries {
            self.entry(&entry);
        }
        self
    }

    pub fn entry(&mut self, entry: &dyn Display) -> &mut Self {
        self.0.entry(&format_args!("{}", entry));
        self
    }

    pub fn finish(&mut self) -> Result {
        self.0.finish()
    }
}

/// A struct to help with [`Display`] implementations
///
/// Same as [`DebugMap`] except that each entry format is [`Display`].
pub struct DisplayMap<'a, 'b>(DebugMap<'a, 'b>);

impl DisplayMap<'_, '_> {
    pub fn entries<K, V, I>(&mut self, entries: I) -> &mut Self
    where
        K: Display,
        V: Display,
        I: IntoIterator<Item = (K, V)>,
    {
        for (key, value) in entries {
            self.entry(&key, &value);
        }
        self
    }

    pub fn entry(&mut self, key: &dyn Display, value: &dyn Display) -> &mut Self {
        self.key(key).value(value)
    }

    pub fn finish(&mut self) -> Result {
        self.0.finish()
    }

    pub fn key(&mut self, key: &dyn Display) -> &mut Self {
        self.0.key(&format_args!("{}", key));
        self
    }

    pub fn value(&mut self, value: &dyn Display) -> &mut Self {
        self.0.value(&format_args!("{}", value));
        self
    }
}

/// A struct to help with [`Display`] implementations
///
/// Same as [`DebugSet`] except that each entry format is [`Display`].
pub struct DisplaySet<'a, 'b>(DebugSet<'a, 'b>);

impl DisplaySet<'_, '_> {
    pub fn entries<T, I>(&mut self, entries: I) -> &mut Self
    where
        T: Display,
        I: IntoIterator<Item = T>,
    {
        for entry in entries {
            self.entry(&entry);
        }
        self
    }

    pub fn entry(&mut self, entry: &dyn Display) -> &mut Self {
        self.0.entry(&format_args!("{}", entry));
        self
    }

    pub fn finish(&mut self) -> Result {
        self.0.finish()
    }
}

/// A struct to help with [`Display`] implementations
///
/// Same as [`DebugStruct`] except that each field value format is [`Display`].
pub struct DisplayStruct<'a, 'b>(DebugStruct<'a, 'b>);

impl DisplayStruct<'_, '_> {
    pub fn field(&mut self, name: &str, value: &dyn Display) -> &mut Self {
        self.0.field(name, &format_args!("{}", value));
        self
    }

    // TODO:
    // pub fn finish_non_exhaustive(&mut self) -> Result {
    //     self.0.finish_non_exhaustive()
    // }

    pub fn finish(&mut self) -> Result {
        self.0.finish()
    }
}

/// A struct to help with [`Display`] implementations
///
/// Same as [`DebugTuple`] except that each field format is [`Display`].
pub struct DisplayTuple<'a, 'b>(DebugTuple<'a, 'b>);

impl DisplayTuple<'_, '_> {
    pub fn field(&mut self, value: &dyn Display) -> &mut Self {
        self.0.field(&format_args!("{}", value));
        self
    }

    pub fn finish(&mut self) -> Result {
        self.0.finish()
    }
}
