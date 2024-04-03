use reqwest::header::{self, HeaderMap, HeaderName, HeaderValue, IntoHeaderName};

/// Builder for constructing a `HeaderMap` instance with various headers.
#[derive(Debug, Clone)]
pub struct HeaderMapBuilder {
    map: HeaderMap,
}

impl HeaderMapBuilder {
    /// Creates a new `HeaderMapBuilder` instance.
    pub fn new() -> Self {
        Self {
            map: HeaderMap::new(),
        }
    }

    /// Builds and returns the constructed `HeaderMap`.
    pub fn build(self) -> HeaderMap {
        self.map
    }

    /// Adds a header field with the specified key and value.
    pub fn field<K: IntoHeaderName, V: Into<HeaderBuilderValue>>(mut self, key: K, val: V) -> Self {
        self.map.insert(key, val.into().into());
        self
    }

    /// Adds a 'User-Agent' header with a default value.
    pub fn with_user_agent(self) -> Self {
        self.field(
            header::USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:84.0) Gecko/20100101 Firefox/84.0",
        )
    }

    /// Sets the 'User-Agent' header with the specified value.
    pub fn set_user_agent<V: Into<HeaderBuilderValue>>(self, user_agent_value: V) -> Self {
        self.field(header::USER_AGENT, user_agent_value)
    }

    /// Adds an 'Accept' header with the specified value.
    pub fn accept<V: Into<HeaderBuilderValue>>(self, user_agent_value: V) -> Self {
        self.field(header::ACCEPT, user_agent_value)
    }

    /// Adds an 'Accept-Language' header with the specified value.
    pub fn accept_language<V: Into<HeaderBuilderValue>>(self, user_agent_value: V) -> Self {
        self.field(header::ACCEPT_LANGUAGE, user_agent_value)
    }

    /// Adds a 'TE' header with the specified value.
    pub fn te<V: Into<HeaderBuilderValue>>(self, user_agent_value: V) -> Self {
        self.field(header::TE, user_agent_value)
    }

    /// Adds an 'Authorization' header with the specified value.
    pub fn authorization<V: Into<HeaderBuilderValue>>(self, user_agent_value: V) -> Self {
        self.field(header::AUTHORIZATION, user_agent_value)
    }

    /// Adds a 'Content-Type' header with the specified value.
    pub fn content_type<V: Into<HeaderBuilderValue>>(self, user_agent_value: V) -> Self {
        self.field(header::CONTENT_TYPE, user_agent_value)
    }

    /// Adds a 'Content-Length' header with the specified value.
    pub fn content_length<V: Into<HeaderBuilderValue>>(self, user_agent_value: V) -> Self {
        self.field(header::CONTENT_LENGTH, user_agent_value)
    }

    /// Adds a 'Referer' header with the specified value.
    pub fn referer<V: Into<HeaderBuilderValue>>(self, user_agent_value: V) -> Self {
        self.field(header::REFERER, user_agent_value)
    }

    /// Adds an 'Origin' header with the specified value.
    pub fn origin<V: Into<HeaderBuilderValue>>(self, user_agent_value: V) -> Self {
        self.field(header::ORIGIN, user_agent_value)
    }

    /// Adds a 'Host' header with the specified value.
    pub fn host<V: Into<HeaderBuilderValue>>(self, user_agent_value: V) -> Self {
        self.field(header::HOST, user_agent_value)
    }

    /// Adds an 'Accept-Encoding' header with the specified value.
    pub fn accept_encoding<V: Into<HeaderBuilderValue>>(self, user_agent_value: V) -> Self {
        self.field(header::ACCEPT_ENCODING, user_agent_value)
    }

    /// Adds a 'Connection' header with the specified value.
    pub fn connection<V: Into<HeaderBuilderValue>>(self, user_agent_value: V) -> Self {
        self.field(header::CONNECTION, user_agent_value)
    }

    /// Adds a 'Cache-Control' header with the specified value.
    pub fn cache_control<V: Into<HeaderBuilderValue>>(self, value: V) -> Self {
        self.field(header::CACHE_CONTROL, value)
    }

    /// Adds a 'Pragma' header with the specified value.
    pub fn pragma<V: Into<HeaderBuilderValue>>(self, user_agent_value: V) -> Self {
        self.field(header::PRAGMA, user_agent_value)
    }

    /// Adds a 'Content-Disposition' header with the specified value.
    pub fn content_disposition<V: Into<HeaderBuilderValue>>(self, user_agent_value: V) -> Self {
        self.field(header::CONTENT_DISPOSITION, user_agent_value)
    }
}

#[derive(Debug, Clone)]
pub enum HeaderBuilderValue {
    Static(&'static str),
    Value(HeaderValue),
    Name(HeaderName),
}

impl Into<HeaderValue> for HeaderBuilderValue {
    fn into(self) -> HeaderValue {
        match self {
            HeaderBuilderValue::Static(s) => HeaderValue::from_static(s),
            HeaderBuilderValue::Value(v) => v,
            HeaderBuilderValue::Name(name) => HeaderValue::from_name(name),
        }
    }
}

impl From<&'static str> for HeaderBuilderValue {
    fn from(value: &'static str) -> Self {
        HeaderBuilderValue::Static(value)
    }
}
impl From<HeaderValue> for HeaderBuilderValue {
    fn from(value: HeaderValue) -> Self {
        HeaderBuilderValue::Value(value)
    }
}
impl From<HeaderName> for HeaderBuilderValue {
    fn from(value: HeaderName) -> Self {
        HeaderBuilderValue::Name(value)
    }
}
