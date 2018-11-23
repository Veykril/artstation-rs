macro_rules! make_request {
    () => {};
    ($name:ident = $response:ident; $($tail:tt)*) => {
        pub struct $name;
        impl crate::request::ArtStationRequest for $name {
            type Response = $response;
        }
        make_request!($($tail)*);
    };
    ($name:ident = $response:ident<$inner:ident>; $($tail:tt)*) => {
        pub struct $name;
        impl crate::request::ArtStationRequest for $name {
            type Response = $response<$inner>;
        }
        make_request!($($tail)*);
    };
    ($name:ident = $response:ident with $($query:ident),*; $($tail:tt)*) => {
        make_request!($name = $response;);
        $(
            impl $query for $name {}
        )*
        make_request!($($tail)*);
    };
    ($name:ident = $response:ident<$inner:ident> with $($query:ident),*; $($tail:tt)*) => {
        make_request!($name = $response<$inner>;);
        $(
            impl $query for $name {}
        )*
        make_request!($($tail)*);
    };
}

// we need this instead of blanket implementing on DeserializeOwned cause of specialization cases
macro_rules! impl_generic_json_response {
    ($($name:ident),*) => {
        $(
            impl crate::request::response::ArtStationResponse for $name {
                type Output = Self;
                fn from_reqwest_response(mut response: crate::reqwest::Response) -> reqwest::Result<Self> {
                    Ok(response.json()?)
                }
            }
        )*
    };
}
