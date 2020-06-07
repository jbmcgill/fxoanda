pub mod get_instrument_candles {
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
        #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
        pub price: Option<String>,

        #[serde(rename = "granularity", skip_serializing_if = "Option::is_none")]
        pub granularity: Option<CandlestickGranularity>,

        #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
        pub count: Option<i32>,

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

        #[serde(rename = "smooth", skip_serializing_if = "Option::is_none")]
        pub smooth: Option<bool>,

        #[serde(rename = "includeFirst", skip_serializing_if = "Option::is_none")]
        pub include_first: Option<bool>,

        #[serde(rename = "dailyAlignment", skip_serializing_if = "Option::is_none")]
        pub daily_alignment: Option<i32>,

        #[serde(rename = "alignmentTimezone", skip_serializing_if = "Option::is_none")]
        pub alignment_timezone: Option<String>,

        #[serde(rename = "weeklyAlignment", skip_serializing_if = "Option::is_none")]
        pub weekly_alignment: Option<String>,
    }
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {
                price: None,
                granularity: None,
                count: None,
                from: None,
                to: None,
                smooth: None,
                include_first: None,
                daily_alignment: None,
                alignment_timezone: None,
                weekly_alignment: None,
            }
        }
    }

    /// Get Candlesticks
    /// Fetch candlestick data for an instrument.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetInstrumentCandlesRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetInstrumentCandlesRequest {
        pub fn new() -> GetInstrumentCandlesRequest {
            GetInstrumentCandlesRequest {
                uri: String::from("/v3/instruments/{instrument}/candles"),
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
        /// - return GetInstrumentCandlesRequest
        pub fn with_instrument(mut self, x: String) -> Self {
            self.path.instrument = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetInstrumentCandlesRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return GetInstrumentCandlesRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// The Price component(s) to get candlestick data for. Can contain any
        /// combination of the characters "M" (midpoint candles) "B" (bid candles)
        /// and "A" (ask candles).
        /// - param String
        /// - return GetInstrumentCandlesRequest
        pub fn with_price(mut self, x: String) -> Self {
            self.query.price = Some(x);
            self
        }

        /// The granularity of the candlesticks to fetch
        /// - param CandlestickGranularity
        /// - return GetInstrumentCandlesRequest
        pub fn with_granularity(mut self, x: CandlestickGranularity) -> Self {
            self.query.granularity = Some(x);
            self
        }

        /// The number of candlesticks to return in the reponse. Count should not
        /// be specified if both the start and end parameters are provided, as the
        /// time range combined with the graularity will determine the number of
        /// candlesticks to return.
        /// - param i32
        /// - return GetInstrumentCandlesRequest
        pub fn with_count(mut self, x: i32) -> Self {
            self.query.count = Some(x);
            self
        }

        /// The start of the time range to fetch candlesticks for.
        /// format: The RFC 3339 representation is a string conforming to
        /// https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a
        /// string representing the number of seconds since the Unix Epoch
        /// (January 1st, 1970 at UTC). The value is a fractional number, where
        /// the fractional part represents a fraction of a second (up to nine
        /// decimal places).
        /// - param DateTime<Utc>
        /// - return GetInstrumentCandlesRequest
        pub fn with_from(mut self, x: DateTime<Utc>) -> Self {
            self.query.from = Some(x);
            self
        }

        /// The end of the time range to fetch candlesticks for.
        /// format: The RFC 3339 representation is a string conforming to
        /// https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a
        /// string representing the number of seconds since the Unix Epoch
        /// (January 1st, 1970 at UTC). The value is a fractional number, where
        /// the fractional part represents a fraction of a second (up to nine
        /// decimal places).
        /// - param DateTime<Utc>
        /// - return GetInstrumentCandlesRequest
        pub fn with_to(mut self, x: DateTime<Utc>) -> Self {
            self.query.to = Some(x);
            self
        }

        /// A flag that controls whether the candlestick is "smoothed" or not.  A
        /// smoothed candlestick uses the previous candle's close price as its
        /// open price, while an unsmoothed candlestick uses the first price from
        /// its time range as its open price.
        /// - param bool
        /// - return GetInstrumentCandlesRequest
        pub fn with_smooth(mut self, x: bool) -> Self {
            self.query.smooth = Some(x);
            self
        }

        /// A flag that controls whether the candlestick that is covered by the
        /// from time should be included in the results. This flag enables clients
        /// to use the timestamp of the last completed candlestick received to
        /// poll for future candlesticks but avoid receiving the previous
        /// candlestick repeatedly.
        /// - param bool
        /// - return GetInstrumentCandlesRequest
        pub fn with_include_first(mut self, x: bool) -> Self {
            self.query.include_first = Some(x);
            self
        }

        /// The hour of the day (in the specified timezone) to use for
        /// granularities that have daily alignments.
        /// - param i32
        /// - return GetInstrumentCandlesRequest
        pub fn with_daily_alignment(mut self, x: i32) -> Self {
            self.query.daily_alignment = Some(x);
            self
        }

        /// The timezone to use for the dailyAlignment parameter. Candlesticks
        /// with daily alignment will be aligned to the dailyAlignment hour within
        /// the alignmentTimezone.  Note that the returned times will still be
        /// represented in UTC.
        /// - param String
        /// - return GetInstrumentCandlesRequest
        pub fn with_alignment_timezone(mut self, x: String) -> Self {
            self.query.alignment_timezone = Some(x);
            self
        }

        /// The day of the week used for granularities that have weekly alignment.
        /// - param String
        /// - return GetInstrumentCandlesRequest
        pub fn with_weekly_alignment(mut self, x: String) -> Self {
            self.query.weekly_alignment = Some(x);
            self
        }

        pub fn remote(
            self,
            client: &Client,
        ) -> Result<GetInstrumentCandlesResponse, Box<dyn Error>> {
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
                Ok(mut response) => match response.json::<GetInstrumentCandlesResponse>() {
                    Err(e) => Err(Box::new(e)),
                    Ok(j) => Ok(j),
                },
            }
        }
    }

    pub type GetInstrumentCandlesResponse = GetInstrumentCandlesResponse200Body;

    /// Pricing information has been successfully provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetInstrumentCandlesResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// Pricing information has been successfully provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetInstrumentCandlesResponse200Body {
        /// The instrument whose Prices are represented by the candlesticks.
        /// format: A string containing the base currency and quote currency delimited by
        /// a "_".
        #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
        pub instrument: Option<String>,
        /// The granularity of the candlesticks provided.
        #[serde(rename = "granularity", skip_serializing_if = "Option::is_none")]
        pub granularity: Option<CandlestickGranularity>,
        /// The list of candlesticks that satisfy the request.
        #[serde(rename = "candles", skip_serializing_if = "Option::is_none")]
        pub candles: Option<Vec<Candlestick>>,
    }
}

pub mod get_instrument_price {
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

    /// Price
    /// Fetch a price for an instrument. Accounts are not associated in any
    /// way with this endpoint.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetInstrumentPriceRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetInstrumentPriceRequest {
        pub fn new() -> GetInstrumentPriceRequest {
            GetInstrumentPriceRequest {
                uri: String::from("/v3/instruments/{instrument}/price"),
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
        /// - return GetInstrumentPriceRequest
        pub fn with_instrument(mut self, x: String) -> Self {
            self.path.instrument = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetInstrumentPriceRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return GetInstrumentPriceRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// The time at which the desired price is in effect. The current price is
        /// returned if no time is provided.
        /// format: The RFC 3339 representation is a string conforming to
        /// https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a
        /// string representing the number of seconds since the Unix Epoch
        /// (January 1st, 1970 at UTC). The value is a fractional number, where
        /// the fractional part represents a fraction of a second (up to nine
        /// decimal places).
        /// - param DateTime<Utc>
        /// - return GetInstrumentPriceRequest
        pub fn with_time(mut self, x: DateTime<Utc>) -> Self {
            self.query.time = Some(x);
            self
        }

        pub fn remote(self, client: &Client) -> Result<GetInstrumentPriceResponse, Box<dyn Error>> {
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
                Ok(mut response) => match response.json::<GetInstrumentPriceResponse>() {
                    Err(e) => Err(Box::new(e)),
                    Ok(j) => Ok(j),
                },
            }
        }
    }

    pub type GetInstrumentPriceResponse = GetInstrumentPriceResponse200Body;

    /// Pricing information has been successfully provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetInstrumentPriceResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// Pricing information has been successfully provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetInstrumentPriceResponse200Body {
        /// The Price representation
        #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
        pub price: Option<Price>,
    }
}

pub mod get_instrument_price_range {
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

    /// Get Prices
    /// Fetch a range of prices for an instrument. Accounts are not associated
    /// in any way with this endpoint.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetInstrumentPriceRangeRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetInstrumentPriceRangeRequest {
        pub fn new() -> GetInstrumentPriceRangeRequest {
            GetInstrumentPriceRangeRequest {
                uri: String::from("/v3/instruments/{instrument}/price/range"),
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
        /// - return GetInstrumentPriceRangeRequest
        pub fn with_instrument(mut self, x: String) -> Self {
            self.path.instrument = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetInstrumentPriceRangeRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return GetInstrumentPriceRangeRequest
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
        /// - return GetInstrumentPriceRangeRequest
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
        /// - return GetInstrumentPriceRangeRequest
        pub fn with_to(mut self, x: DateTime<Utc>) -> Self {
            self.query.to = Some(x);
            self
        }

        pub fn remote(
            self,
            client: &Client,
        ) -> Result<GetInstrumentPriceRangeResponse, Box<dyn Error>> {
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
                Ok(mut response) => match response.json::<GetInstrumentPriceRangeResponse>() {
                    Err(e) => Err(Box::new(e)),
                    Ok(j) => Ok(j),
                },
            }
        }
    }

    pub type GetInstrumentPriceRangeResponse = GetInstrumentPriceRangeResponse200Body;

    /// Pricing information has been successfully provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetInstrumentPriceRangeResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
        /// A link to the next page of results if the results were paginated
        #[serde(rename = "Link", skip_serializing_if = "Option::is_none")]
        pub link: Option<String>,
    }

    /// Pricing information has been successfully provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetInstrumentPriceRangeResponse200Body {
        /// The list of prices that satisfy the request.
        #[serde(rename = "prices", skip_serializing_if = "Option::is_none")]
        pub prices: Option<Vec<Price>>,
    }
}

pub mod get_order_book {
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

    /// Get Order Book
    /// Fetch an order book for an instrument.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetOrderBookRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetOrderBookRequest {
        pub fn new() -> GetOrderBookRequest {
            GetOrderBookRequest {
                uri: String::from("/v3/instruments/{instrument}/orderBook"),
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
        /// - return GetOrderBookRequest
        pub fn with_instrument(mut self, x: String) -> Self {
            self.path.instrument = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetOrderBookRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return GetOrderBookRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// The time of the snapshot to fetch. If not specified, then the most
        /// recent snapshot is fetched.
        /// format: The RFC 3339 representation is a string conforming to
        /// https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a
        /// string representing the number of seconds since the Unix Epoch
        /// (January 1st, 1970 at UTC). The value is a fractional number, where
        /// the fractional part represents a fraction of a second (up to nine
        /// decimal places).
        /// - param DateTime<Utc>
        /// - return GetOrderBookRequest
        pub fn with_time(mut self, x: DateTime<Utc>) -> Self {
            self.query.time = Some(x);
            self
        }

        pub fn remote(self, client: &Client) -> Result<GetOrderBookResponse, Box<dyn Error>> {
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
                Ok(mut response) => match response.json::<GetOrderBookResponse>() {
                    Err(e) => Err(Box::new(e)),
                    Ok(j) => Ok(j),
                },
            }
        }
    }

    pub type GetOrderBookResponse = GetOrderBookResponse200Body;

    /// The order book has been successfully provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetOrderBookResponse200Header {
        /// Value will be "gzip" regardless of provided Accept-Encoding header
        #[serde(rename = "ContentEncoding", skip_serializing_if = "Option::is_none")]
        pub content_encoding: Option<String>,
        /// A link to the next/previous order book snapshot.
        #[serde(rename = "Link", skip_serializing_if = "Option::is_none")]
        pub link: Option<String>,
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The order book has been successfully provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetOrderBookResponse200Body {
        /// The representation of an instrument's order book at a point in time
        #[serde(rename = "orderBook", skip_serializing_if = "Option::is_none")]
        pub order_book: Option<OrderBook>,
    }
}

pub mod get_position_book {
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

    /// Get Position Book
    /// Fetch a position book for an instrument.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetPositionBookRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetPositionBookRequest {
        pub fn new() -> GetPositionBookRequest {
            GetPositionBookRequest {
                uri: String::from("/v3/instruments/{instrument}/positionBook"),
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
        /// - return GetPositionBookRequest
        pub fn with_instrument(mut self, x: String) -> Self {
            self.path.instrument = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetPositionBookRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return GetPositionBookRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// The time of the snapshot to fetch. If not specified, then the most
        /// recent snapshot is fetched.
        /// format: The RFC 3339 representation is a string conforming to
        /// https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a
        /// string representing the number of seconds since the Unix Epoch
        /// (January 1st, 1970 at UTC). The value is a fractional number, where
        /// the fractional part represents a fraction of a second (up to nine
        /// decimal places).
        /// - param DateTime<Utc>
        /// - return GetPositionBookRequest
        pub fn with_time(mut self, x: DateTime<Utc>) -> Self {
            self.query.time = Some(x);
            self
        }

        pub fn remote(self, client: &Client) -> Result<GetPositionBookResponse, Box<dyn Error>> {
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
                Ok(mut response) => match response.json::<GetPositionBookResponse>() {
                    Err(e) => Err(Box::new(e)),
                    Ok(j) => Ok(j),
                },
            }
        }
    }

    pub type GetPositionBookResponse = GetPositionBookResponse200Body;

    /// The position book has been successfully provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetPositionBookResponse200Header {
        /// Value will be "gzip" regardless of provided Accept-Encoding header
        #[serde(rename = "ContentEncoding", skip_serializing_if = "Option::is_none")]
        pub content_encoding: Option<String>,
        /// A link to the next/previous position book snapshot.
        #[serde(rename = "Link", skip_serializing_if = "Option::is_none")]
        pub link: Option<String>,
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The position book has been successfully provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetPositionBookResponse200Body {
        /// The representation of an instrument's position book at a point in time
        #[serde(rename = "positionBook", skip_serializing_if = "Option::is_none")]
        pub position_book: Option<PositionBook>,
    }
}
pub use get_instrument_candles::*;
pub use get_instrument_price::*;
pub use get_instrument_price_range::*;
pub use get_order_book::*;
pub use get_position_book::*;
