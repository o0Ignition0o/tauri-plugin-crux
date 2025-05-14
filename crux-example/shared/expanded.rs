mod app {
    use chrono::{
        serde::ts_milliseconds_option::deserialize as ts_milliseconds_option, DateTime,
        Utc,
    };
    use crux_core::{
        render::{render, Render},
        Command,
    };
    use crux_http::command::Http;
    use serde::{Deserialize, Serialize};
    use url::Url;
    use crate::sse::ServerSentEvents;
    const API_URL: &str = "https://crux-counter.fly.dev";
    pub struct Model {
        count: Count,
    }
    #[automatically_derived]
    impl ::core::default::Default for Model {
        #[inline]
        fn default() -> Model {
            Model {
                count: ::core::default::Default::default(),
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Model {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Model",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "count",
                    &self.count,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub struct Count {
        value: isize,
        #[serde(deserialize_with = "ts_milliseconds_option")]
        updated_at: Option<DateTime<Utc>>,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Count {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Count",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "value",
                    &self.value,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "updated_at",
                    &self.updated_at,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Count {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "value" => _serde::__private::Ok(__Field::__field0),
                            "updated_at" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"value" => _serde::__private::Ok(__Field::__field0),
                            b"updated_at" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Count>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Count;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Count",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            isize,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Count with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match {
                            #[doc(hidden)]
                            struct __DeserializeWith<'de> {
                                value: Option<DateTime<Utc>>,
                                phantom: _serde::__private::PhantomData<Count>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            #[automatically_derived]
                            impl<'de> _serde::Deserialize<'de>
                            for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::__private::Ok(__DeserializeWith {
                                        value: ts_milliseconds_option(__deserializer)?,
                                        phantom: _serde::__private::PhantomData,
                                        lifetime: _serde::__private::PhantomData,
                                    })
                                }
                            }
                            _serde::__private::Option::map(
                                _serde::de::SeqAccess::next_element::<
                                    __DeserializeWith<'de>,
                                >(&mut __seq)?,
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Count with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Count {
                            value: __field0,
                            updated_at: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<isize> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<
                            Option<DateTime<Utc>>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<isize>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "updated_at",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some({
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: Option<DateTime<Utc>>,
                                            phantom: _serde::__private::PhantomData<Count>,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private::Ok(__DeserializeWith {
                                                    value: ts_milliseconds_option(__deserializer)?,
                                                    phantom: _serde::__private::PhantomData,
                                                    lifetime: _serde::__private::PhantomData,
                                                })
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    });
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("value")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field(
                                        "updated_at",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Count {
                            value: __field0,
                            updated_at: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["value", "updated_at"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Count",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Count>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for Count {
        #[inline]
        fn clone(&self) -> Count {
            Count {
                value: ::core::clone::Clone::clone(&self.value),
                updated_at: ::core::clone::Clone::clone(&self.updated_at),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Count {
        #[inline]
        fn default() -> Count {
            Count {
                value: ::core::default::Default::default(),
                updated_at: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Count {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Count",
                "value",
                &self.value,
                "updated_at",
                &&self.updated_at,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Count {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Count {
        #[inline]
        fn eq(&self, other: &Count) -> bool {
            self.value == other.value && self.updated_at == other.updated_at
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Count {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<isize>;
            let _: ::core::cmp::AssertParamIsEq<Option<DateTime<Utc>>>;
        }
    }
    pub struct ViewModel {
        pub text: String,
        pub confirmed: bool,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ViewModel {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "ViewModel",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "text",
                    &self.text,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "confirmed",
                    &self.confirmed,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ViewModel {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "text" => _serde::__private::Ok(__Field::__field0),
                            "confirmed" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"text" => _serde::__private::Ok(__Field::__field0),
                            b"confirmed" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ViewModel>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ViewModel;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct ViewModel",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct ViewModel with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct ViewModel with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(ViewModel {
                            text: __field0,
                            confirmed: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("text"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "confirmed",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("text")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("confirmed")?
                            }
                        };
                        _serde::__private::Ok(ViewModel {
                            text: __field0,
                            confirmed: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["text", "confirmed"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ViewModel",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ViewModel>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for ViewModel {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ViewModel",
                "text",
                &self.text,
                "confirmed",
                &&self.confirmed,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ViewModel {
        #[inline]
        fn clone(&self) -> ViewModel {
            ViewModel {
                text: ::core::clone::Clone::clone(&self.text),
                confirmed: ::core::clone::Clone::clone(&self.confirmed),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for ViewModel {
        #[inline]
        fn default() -> ViewModel {
            ViewModel {
                text: ::core::default::Default::default(),
                confirmed: ::core::default::Default::default(),
            }
        }
    }
    pub enum Event {
        Get,
        Increment,
        Decrement,
        StartWatch,
        #[serde(skip)]
        Set(crux_http::Result<crux_http::Response<Count>>),
        #[serde(skip)]
        Update(Count),
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Event {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Event::Get => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "Event",
                            0u32,
                            "Get",
                        )
                    }
                    Event::Increment => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "Event",
                            1u32,
                            "Increment",
                        )
                    }
                    Event::Decrement => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "Event",
                            2u32,
                            "Decrement",
                        )
                    }
                    Event::StartWatch => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "Event",
                            3u32,
                            "StartWatch",
                        )
                    }
                    Event::Set(..) => {
                        _serde::__private::Err(
                            _serde::ser::Error::custom(
                                "the enum variant Event::Set cannot be serialized",
                            ),
                        )
                    }
                    Event::Update(..) => {
                        _serde::__private::Err(
                            _serde::ser::Error::custom(
                                "the enum variant Event::Update cannot be serialized",
                            ),
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Event {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 4",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Get" => _serde::__private::Ok(__Field::__field0),
                            "Increment" => _serde::__private::Ok(__Field::__field1),
                            "Decrement" => _serde::__private::Ok(__Field::__field2),
                            "StartWatch" => _serde::__private::Ok(__Field::__field3),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Get" => _serde::__private::Ok(__Field::__field0),
                            b"Increment" => _serde::__private::Ok(__Field::__field1),
                            b"Decrement" => _serde::__private::Ok(__Field::__field2),
                            b"StartWatch" => _serde::__private::Ok(__Field::__field3),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Event>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Event;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum Event",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(Event::Get)
                            }
                            (__Field::__field1, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(Event::Increment)
                            }
                            (__Field::__field2, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(Event::Decrement)
                            }
                            (__Field::__field3, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(Event::StartWatch)
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[
                    "Get",
                    "Increment",
                    "Decrement",
                    "StartWatch",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Event",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Event>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for Event {
        #[inline]
        fn clone(&self) -> Event {
            match self {
                Event::Get => Event::Get,
                Event::Increment => Event::Increment,
                Event::Decrement => Event::Decrement,
                Event::StartWatch => Event::StartWatch,
                Event::Set(__self_0) => Event::Set(::core::clone::Clone::clone(__self_0)),
                Event::Update(__self_0) => {
                    Event::Update(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Event {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Event::Get => ::core::fmt::Formatter::write_str(f, "Get"),
                Event::Increment => ::core::fmt::Formatter::write_str(f, "Increment"),
                Event::Decrement => ::core::fmt::Formatter::write_str(f, "Decrement"),
                Event::StartWatch => ::core::fmt::Formatter::write_str(f, "StartWatch"),
                Event::Set(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Set",
                        &__self_0,
                    )
                }
                Event::Update(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Update",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Event {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Event {
        #[inline]
        fn eq(&self, other: &Event) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (Event::Set(__self_0), Event::Set(__arg1_0)) => __self_0 == __arg1_0,
                    (Event::Update(__self_0), Event::Update(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Event {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                crux_http::Result<crux_http::Response<Count>>,
            >;
            let _: ::core::cmp::AssertParamIsEq<Count>;
        }
    }
    pub struct Capabilities {
        pub render: Render<Event>,
        pub http: crux_http::Http<Event>,
        pub sse: ServerSentEvents<Event>,
    }
    pub enum Effect {
        Http(
            ::crux_core::Request<
                <crux_http::Http<
                    Event,
                > as ::crux_core::capability::Capability<Event>>::Operation,
            >,
        ),
        Render(
            ::crux_core::Request<
                <Render<Event> as ::crux_core::capability::Capability<Event>>::Operation,
            >,
        ),
        ServerSentEvents(
            ::crux_core::Request<
                <ServerSentEvents<
                    Event,
                > as ::crux_core::capability::Capability<Event>>::Operation,
            >,
        ),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Effect {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Effect::Http(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Http",
                        &__self_0,
                    )
                }
                Effect::Render(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Render",
                        &__self_0,
                    )
                }
                Effect::ServerSentEvents(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ServerSentEvents",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[serde(rename = "Effect")]
    pub enum EffectFfi {
        Http(
            <crux_http::Http<
                Event,
            > as ::crux_core::capability::Capability<Event>>::Operation,
        ),
        Render(<Render<Event> as ::crux_core::capability::Capability<Event>>::Operation),
        ServerSentEvents(
            <ServerSentEvents<
                Event,
            > as ::crux_core::capability::Capability<Event>>::Operation,
        ),
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for EffectFfi {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    EffectFfi::Http(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Effect",
                            0u32,
                            "Http",
                            __field0,
                        )
                    }
                    EffectFfi::Render(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Effect",
                            1u32,
                            "Render",
                            __field0,
                        )
                    }
                    EffectFfi::ServerSentEvents(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Effect",
                            2u32,
                            "ServerSentEvents",
                            __field0,
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for EffectFfi {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 3",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Http" => _serde::__private::Ok(__Field::__field0),
                            "Render" => _serde::__private::Ok(__Field::__field1),
                            "ServerSentEvents" => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Http" => _serde::__private::Ok(__Field::__field0),
                            b"Render" => _serde::__private::Ok(__Field::__field1),
                            b"ServerSentEvents" => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<EffectFfi>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = EffectFfi;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum EffectFfi",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::__private::Result::map(
                                    _serde::de::VariantAccess::newtype_variant::<
                                        <crux_http::Http<
                                            Event,
                                        > as ::crux_core::capability::Capability<Event>>::Operation,
                                    >(__variant),
                                    EffectFfi::Http,
                                )
                            }
                            (__Field::__field1, __variant) => {
                                _serde::__private::Result::map(
                                    _serde::de::VariantAccess::newtype_variant::<
                                        <Render<
                                            Event,
                                        > as ::crux_core::capability::Capability<Event>>::Operation,
                                    >(__variant),
                                    EffectFfi::Render,
                                )
                            }
                            (__Field::__field2, __variant) => {
                                _serde::__private::Result::map(
                                    _serde::de::VariantAccess::newtype_variant::<
                                        <ServerSentEvents<
                                            Event,
                                        > as ::crux_core::capability::Capability<Event>>::Operation,
                                    >(__variant),
                                    EffectFfi::ServerSentEvents,
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[
                    "Http",
                    "Render",
                    "ServerSentEvents",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Effect",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<EffectFfi>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl ::crux_core::Effect for Effect {
        type Ffi = EffectFfi;
        fn serialize(self) -> (Self::Ffi, ::crux_core::bridge::ResolveSerialized) {
            match self {
                Effect::Http(request) => request.serialize(EffectFfi::Http),
                Effect::Render(request) => request.serialize(EffectFfi::Render),
                Effect::ServerSentEvents(request) => {
                    request.serialize(EffectFfi::ServerSentEvents)
                }
            }
        }
    }
    impl ::crux_core::WithContext<Event, Effect> for Capabilities {
        fn new_with_context(
            context: ::crux_core::capability::ProtoContext<Effect, Event>,
        ) -> Capabilities {
            Capabilities {
                http: crux_http::Http::new(context.specialize(Effect::Http)),
                render: Render::new(context.specialize(Effect::Render)),
                sse: ServerSentEvents::new(context.specialize(Effect::ServerSentEvents)),
            }
        }
    }
    impl Effect {
        pub fn is_http(&self) -> bool {
            if let Effect::Http(_) = self { true } else { false }
        }
        pub fn into_http(
            self,
        ) -> Option<
            crux_core::Request<
                <crux_http::Http<
                    Event,
                > as ::crux_core::capability::Capability<Event>>::Operation,
            >,
        > {
            if let Effect::Http(request) = self { Some(request) } else { None }
        }
        #[track_caller]
        pub fn expect_http(
            self,
        ) -> crux_core::Request<
            <crux_http::Http<
                Event,
            > as ::crux_core::capability::Capability<Event>>::Operation,
        > {
            if let Effect::Http(request) = self {
                request
            } else {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("not a {0} effect", "http"),
                    );
                }
            }
        }
    }
    impl From<
        crux_core::Request<
            <crux_http::Http<
                Event,
            > as ::crux_core::capability::Capability<Event>>::Operation,
        >,
    > for Effect {
        fn from(
            value: crux_core::Request<
                <crux_http::Http<
                    Event,
                > as ::crux_core::capability::Capability<Event>>::Operation,
            >,
        ) -> Self {
            Self::Http(value)
        }
    }
    impl Effect {
        pub fn is_render(&self) -> bool {
            if let Effect::Render(_) = self { true } else { false }
        }
        pub fn into_render(
            self,
        ) -> Option<
            crux_core::Request<
                <Render<Event> as ::crux_core::capability::Capability<Event>>::Operation,
            >,
        > {
            if let Effect::Render(request) = self { Some(request) } else { None }
        }
        #[track_caller]
        pub fn expect_render(
            self,
        ) -> crux_core::Request<
            <Render<Event> as ::crux_core::capability::Capability<Event>>::Operation,
        > {
            if let Effect::Render(request) = self {
                request
            } else {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("not a {0} effect", "render"),
                    );
                }
            }
        }
    }
    impl From<
        crux_core::Request<
            <Render<Event> as ::crux_core::capability::Capability<Event>>::Operation,
        >,
    > for Effect {
        fn from(
            value: crux_core::Request<
                <Render<Event> as ::crux_core::capability::Capability<Event>>::Operation,
            >,
        ) -> Self {
            Self::Render(value)
        }
    }
    impl Effect {
        pub fn is_sse(&self) -> bool {
            if let Effect::ServerSentEvents(_) = self { true } else { false }
        }
        pub fn into_sse(
            self,
        ) -> Option<
            crux_core::Request<
                <ServerSentEvents<
                    Event,
                > as ::crux_core::capability::Capability<Event>>::Operation,
            >,
        > {
            if let Effect::ServerSentEvents(request) = self {
                Some(request)
            } else {
                None
            }
        }
        #[track_caller]
        pub fn expect_sse(
            self,
        ) -> crux_core::Request<
            <ServerSentEvents<
                Event,
            > as ::crux_core::capability::Capability<Event>>::Operation,
        > {
            if let Effect::ServerSentEvents(request) = self {
                request
            } else {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("not a {0} effect", "sse"),
                    );
                }
            }
        }
    }
    impl From<
        crux_core::Request<
            <ServerSentEvents<
                Event,
            > as ::crux_core::capability::Capability<Event>>::Operation,
        >,
    > for Effect {
        fn from(
            value: crux_core::Request<
                <ServerSentEvents<
                    Event,
                > as ::crux_core::capability::Capability<Event>>::Operation,
            >,
        ) -> Self {
            Self::ServerSentEvents(value)
        }
    }
    pub struct App;
    #[automatically_derived]
    impl ::core::default::Default for App {
        #[inline]
        fn default() -> App {
            App {}
        }
    }
    impl crux_core::App for App {
        type Model = Model;
        type Event = Event;
        type ViewModel = ViewModel;
        type Capabilities = Capabilities;
        type Effect = Effect;
        fn update(
            &self,
            msg: Self::Event,
            model: &mut Self::Model,
            _caps: &Self::Capabilities,
        ) -> Command<Effect, Event> {
            self.update(msg, model)
        }
        fn view(&self, model: &Self::Model) -> Self::ViewModel {
            let suffix = match model.count.updated_at {
                None => " (pending)".to_string(),
                Some(d) => {
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(format_args!(" ({0})", d));
                        res
                    })
                }
            };
            Self::ViewModel {
                text: model.count.value.to_string() + &suffix,
                confirmed: model.count.updated_at.is_some(),
            }
        }
    }
    impl App {
        fn update(&self, msg: Event, model: &mut Model) -> Command<Effect, Event> {
            match msg {
                Event::Get => {
                    Http::get(API_URL).expect_json().build().then_send(Event::Set)
                }
                Event::Set(Ok(mut response)) => {
                    let count = response.take_body().unwrap();
                    Command::event(Event::Update(count))
                }
                Event::Set(Err(e)) => {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("Oh no something went wrong: {0:?}", e),
                        );
                    };
                }
                Event::Update(count) => {
                    model.count = count;
                    render()
                }
                Event::Increment => {
                    model.count = Count {
                        value: model.count.value + 1,
                        updated_at: None,
                    };
                    let call_api = {
                        let base = Url::parse(API_URL).unwrap();
                        let url = base.join("/inc").unwrap();
                        Http::post(url).expect_json().build().then_send(Event::Set)
                    };
                    render().and(call_api)
                }
                Event::Decrement => {
                    model.count = Count {
                        value: model.count.value - 1,
                        updated_at: None,
                    };
                    let call_api = {
                        let base = Url::parse(API_URL).unwrap();
                        let url = base.join("/dec").unwrap();
                        Http::post(url).expect_json().build().then_send(Event::Set)
                    };
                    render().and(call_api)
                }
                Event::StartWatch => {
                    let base = Url::parse(API_URL).unwrap();
                    let url = base.join("/sse").unwrap();
                    ServerSentEvents::get(url).then_send(Event::Update)
                }
            }
        }
    }
}
