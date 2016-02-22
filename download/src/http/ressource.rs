// Copyright 2016 Christopher Gundler <c.gundler@mail.de>

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ::Ressource;
use ::Url;
use ::serialize::*;

use std::path::{Path, PathBuf};

pub struct HttpRessource {
    url : Url,
    path : PathBuf
}

impl HttpRessource {
    pub fn set_url(&mut self, url : Url) {
        self.url = url;
    }

    pub fn set_path(&mut self, path : PathBuf) {
        self.path = path;
    }
}

impl Ressource for HttpRessource {
    fn new(url : Url, path : PathBuf) -> Option<HttpRessource> {
        if url.scheme == "http" {
            Some(HttpRessource {
                url : url,
                path : path
            })
        }
        else {
            None
        }
    }

    fn url(&self) -> &Url {
        &self.url
    }

    fn path(&self) -> &Path {
        &self.path.as_path()
    }
}

impl Serialize for HttpRessource {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        serializer.visit_struct("Ressource", StructSerializer::new(self))
    }
}

impl<'a> SerializeMapVisitor for StructSerializer<'a, HttpRessource> {
    fn visit<S: Serializer>(&mut self, visitor: &mut S) -> Result<Option<()>, S::Error> {
        match self.next() {
            0 => Ok(Some(try!(visitor.visit_map_elt("url", &self.value.url().serialize())))),
            1 => Ok(Some(try!(visitor.visit_map_elt("path", &self.value.path())))),
            _ => Ok(None),
        }
    }
}

struct RessourceKeyVisitor;
impl Visitor for RessourceKeyVisitor {
    type Value = ResourceKey;

    fn visit_str<E>(&mut self, value: &str) -> Result<ResourceKey, E> where E: DeserializeError
    {
        match value {
            "path" => Ok(ResourceKey::Path),
            "url" => Ok(ResourceKey::Url),
            _ => Err(DeserializeError::syntax("expected url or path")),
        }
    }
}

struct RessourceVisitor;
impl Visitor for RessourceVisitor {
    type Value = HttpRessource;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<HttpRessource, V::Error> where V: DeserializeMapVisitor
    {
        let mut url = None;
        let mut path = None;

        while let Some(key) = try!(visitor.visit_key()) {
            let value = try!(visitor.visit_value::<String>());
            match key {
                ResourceKey::Path if !value.is_empty() => {
                    path = Some(PathBuf::from(value));
                },
                ResourceKey::Url => {
                    url = Url::parse(&value).ok();
                },
                _ => { }
            }
        }

        if let None = path {
             return visitor.missing_field("path");
        }

        if let None = url {
             return visitor.missing_field("url");
        }

        try!(visitor.end());
        Ok(HttpRessource { path: path.expect("Valid path"), url: url.expect("Valid url") })
    }
}

enum ResourceKey {
    Path,
    Url
}

impl Deserialize for ResourceKey {
    fn deserialize<D>(deserializer: &mut D) -> Result<ResourceKey, D::Error> where D: Deserializer
    {
        deserializer.visit(RessourceKeyVisitor)
    }
}

impl Deserialize for HttpRessource {
    fn deserialize<D>(deserializer: &mut D) -> Result<HttpRessource, D::Error> where D: Deserializer {
        static FIELDS: &'static [&'static str] = &["path", "url"];
        deserializer.visit_struct("Ressource", FIELDS, RessourceVisitor)
    }
}
