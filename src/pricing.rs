pub mod get_base_prices {
    use chrono::prelude::*;
    use fxoanda_definitions::*;
    use fxoanda_serdes::*;
    use std::error::Error;
    use Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(
            rename = "AcceptDatetimeFormat",
            skip_serializing_if = "Option::is_none"
        )]
        pub accept_datetime_format: Option<String>,
    }
    impl RequestHead {
        fn new() -> RequestHead {
            RequestHead {
                authorization: None,
                accept_datetime_format: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestPath {}
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath {}
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestBody {}
    impl RequestBody {
        fn new() -> RequestBody {
            RequestBody {}
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestQuery {
        #[serde(
            rename = "time",
            skip_serializing_if = "Option::is_none",
            with = "serdates"
        )]
        pub time: Option<DateTime<Utc>>,
    }
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery { time: None }
        }
    }

    /// Get Base Prices
    /// Get pricing information for a specified instrument. Accounts are not
    /// associated in any way with this endpoint.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetBasePricesRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetBasePricesRequest {
        pub fn new() -> GetBasePricesRequest {
            GetBasePricesRequest {
                uri: String::from("/v3/pricing"),
                header: RequestHead::new(),
                body: RequestBody::new(),
                path: RequestPath::new(),
                query: RequestQuery::new(),
            }
        }
        pub fn with_uri(mut self, x: String) -> Self {
            self.uri = x;
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetBasePricesRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return GetBasePricesRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// The time at which the desired price for each instrument is in effect.
        /// The current price for each instrument is returned if no time is
        /// provided.
        /// format: The RFC 3339 representation is a string conforming to
        /// https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a
        /// string representing the number of seconds since the Unix Epoch
        /// (January 1st, 1970 at UTC). The value is a fractional number, where
        /// the fractional part represents a fraction of a second (up to nine
        /// decimal places).
        /// - param DateTime<Utc>
        /// - return GetBasePricesRequest
        pub fn with_time(mut self, x: DateTime<Utc>) -> Self {
            self.query.time = Some(x);
            self
        }

        pub fn remote(self, client: &Client) -> Result<GetBasePricesResponse, Box<dyn Error>> {
            let uri = self.uri.clone();
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client
                .reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send();
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(mut response) => match response.json::<GetBasePricesResponse>() {
                    Err(e) => Err(Box::new(e)),
                    Ok(j) => Ok(j),
                },
            }
        }
    }

    pub type GetBasePricesResponse = GetBasePricesResponse200Body;

    /// Pricing information has been successfully provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetBasePricesResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// Pricing information has been successfully provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetBasePricesResponse200Body {
        /// The list of prices that satisfy the request.
        #[serde(rename = "prices", skip_serializing_if = "Option::is_none")]
        pub prices: Option<Vec<Price>>,
    }
}

pub mod get_price_range {
    use chrono::prelude::*;
    use fxoanda_definitions::*;
    use fxoanda_serdes::*;
    use std::error::Error;
    use Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(
            rename = "AcceptDatetimeFormat",
            skip_serializing_if = "Option::is_none"
        )]
        pub accept_datetime_format: Option<String>,
    }
    impl RequestHead {
        fn new() -> RequestHead {
            RequestHead {
                authorization: None,
                accept_datetime_format: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestPath {
        #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
        pub instrument: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { instrument: None }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestBody {}
    impl RequestBody {
        fn new() -> RequestBody {
            RequestBody {}
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestQuery {
        #[serde(
            rename = "from",
            skip_serializing_if = "Option::is_none",
            with = "serdates"
        )]
        pub from: Option<DateTime<Utc>>,

        #[serde(
            rename = "to",
            skip_serializing_if = "Option::is_none",
            with = "serdates"
        )]
        pub to: Option<DateTime<Utc>>,
    }
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {
                from: None,
                to: None,
            }
        }
    }

    /// Get Price Range
    /// Get pricing information for a specified range of prices. Accounts are
    /// not associated in any way with this endpoint.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetPriceRangeRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetPriceRangeRequest {
        pub fn new() -> GetPriceRangeRequest {
            GetPriceRangeRequest {
                uri: String::from("/v3/pricing/range"),
                header: RequestHead::new(),
                body: RequestBody::new(),
                path: RequestPath::new(),
                query: RequestQuery::new(),
            }
        }
        pub fn with_uri(mut self, x: String) -> Self {
            self.uri = x;
            self
        }

        /// Name of the Instrument
        /// format: A string containing the base currency and quote currency delimited by
        /// a "_".
        /// - param String
        /// - return GetPriceRangeRequest
        pub fn with_instrument(mut self, x: String) -> Self {
            self.path.instrument = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetPriceRangeRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return GetPriceRangeRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// The start of the time range to fetch prices for.
        /// format: The RFC 3339 representation is a string conforming to
        /// https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a
        /// string representing the number of seconds since the Unix Epoch
        /// (January 1st, 1970 at UTC). The value is a fractional number, where
        /// the fractional part represents a fraction of a second (up to nine
        /// decimal places).
        /// - param DateTime<Utc>
        /// - return GetPriceRangeRequest
        pub fn with_from(mut self, x: DateTime<Utc>) -> Self {
            self.query.from = Some(x);
            self
        }

        /// The end of the time range to fetch prices for. The current time is
        /// used if this parameter is not provided.
        /// format: The RFC 3339 representation is a string conforming to
        /// https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a
        /// string representing the number of seconds since the Unix Epoch
        /// (January 1st, 1970 at UTC). The value is a fractional number, where
        /// the fractional part represents a fraction of a second (up to nine
        /// decimal places).
        /// - param DateTime<Utc>
        /// - return GetPriceRangeRequest
        pub fn with_to(mut self, x: DateTime<Utc>) -> Self {
            self.query.to = Some(x);
            self
        }

        pub fn remote(self, client: &Client) -> Result<GetPriceRangeResponse, Box<dyn Error>> {
            let uri = self
                .uri
                .clone()
                .replace("{instrument}", &self.path.instrument.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client
                .reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send();
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(mut response) => match response.json::<GetPriceRangeResponse>() {
                    Err(e) => Err(Box::new(e)),
                    Ok(j) => Ok(j),
                },
            }
        }
    }

    pub type GetPriceRangeResponse = GetPriceRangeResponse200Body;

    /// Pricing information has been successfully provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetPriceRangeResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
        /// A link to the next page of results if the results were paginated
        #[serde(rename = "Link", skip_serializing_if = "Option::is_none")]
        pub link: Option<String>,
    }

    /// Pricing information has been successfully provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetPriceRangeResponse200Body {
        /// The list of prices that satisfy the request.
        #[serde(rename = "prices", skip_serializing_if = "Option::is_none")]
        pub prices: Option<Vec<Price>>,
    }
}
pub use get_base_prices::*;
pub use get_price_range::*;
