pub mod list_positions {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,
    }
    impl RequestHead {
        fn new() -> RequestHead {
            RequestHead {
                authorization: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestPath {
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
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
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// List Positions
    /// List all Positions for an Account. The Positions returned are for
    /// every instrument that has had a position during the lifetime of an the
    /// Account.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListPositionsRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl ListPositionsRequest {
        pub fn new() -> ListPositionsRequest {
            ListPositionsRequest {
                uri: String::from("/v3/accounts/{accountID}/positions"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return ListPositionsRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return ListPositionsRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<ListPositionsResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<ListPositionsResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type ListPositionsResponse = ListPositionsResponse200Body;

    /// The Account's Positions are provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListPositionsResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The Account's Positions are provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListPositionsResponse200Body {
        /// The list of Account Positions.
        #[serde(rename = "positions", skip_serializing_if = "Option::is_none")]
        pub positions: Option<Vec<Position>>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod list_open_positions {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,
    }
    impl RequestHead {
        fn new() -> RequestHead {
            RequestHead {
                authorization: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestPath {
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
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
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Open Positions
    /// List all open Positions for an Account. An open Position is a Position
    /// in an Account that currently has a Trade opened for it.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListOpenPositionsRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl ListOpenPositionsRequest {
        pub fn new() -> ListOpenPositionsRequest {
            ListOpenPositionsRequest {
                uri: String::from("/v3/accounts/{accountID}/openPositions"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return ListOpenPositionsRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return ListOpenPositionsRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<ListOpenPositionsResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<ListOpenPositionsResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type ListOpenPositionsResponse = ListOpenPositionsResponse200Body;

    /// The Account's open Positions are provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListOpenPositionsResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The Account's open Positions are provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListOpenPositionsResponse200Body {
        /// The list of open Positions in the Account.
        #[serde(rename = "positions", skip_serializing_if = "Option::is_none")]
        pub positions: Option<Vec<Position>>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod get_position {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,
    }
    impl RequestHead {
        fn new() -> RequestHead {
            RequestHead {
                authorization: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestPath {
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
        pub instrument: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath {
                account_id: None,
                instrument: None,
            }
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
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Instrument Position
    /// Get the details of a single Instrument's Position in an Account. The
    /// Position may by open or not.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetPositionRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetPositionRequest {
        pub fn new() -> GetPositionRequest {
            GetPositionRequest {
                uri: String::from("/v3/accounts/{accountID}/positions/{instrument}"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return GetPositionRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// Name of the Instrument
        /// format: A string containing the base currency and quote currency delimited by
        /// a "_".
        /// - param String
        /// - return GetPositionRequest
        pub fn with_instrument(mut self, x: String) -> Self {
            self.path.instrument = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetPositionRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        pub async fn remote(self, client: &Client) -> Result<GetPositionResponse, Box<dyn Error>> {
            let uri = self.uri
                .clone()
                .replace("{accountID}", &self.path.account_id.unwrap())
                .replace("{instrument}", &self.path.instrument.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<GetPositionResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type GetPositionResponse = GetPositionResponse200Body;

    /// The Position is provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetPositionResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The Position is provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetPositionResponse200Body {
        /// The specification of a Position within an Account.
        #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
        pub position: Option<Position>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod close_position {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
        pub instrument: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath {
                account_id: None,
                instrument: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestBody {
        #[serde(rename = "longUnits", skip_serializing_if = "Option::is_none")]
        pub long_units: Option<String>,

        #[serde(rename = "longClientExtensions", skip_serializing_if = "Option::is_none")]
        pub long_client_extensions: Option<ClientExtensions>,

        #[serde(rename = "shortUnits", skip_serializing_if = "Option::is_none")]
        pub short_units: Option<String>,

        #[serde(rename = "shortClientExtensions", skip_serializing_if = "Option::is_none")]
        pub short_client_extensions: Option<ClientExtensions>,
    }
    impl RequestBody {
        fn new() -> RequestBody {
            RequestBody {
                long_units: None,
                long_client_extensions: None,
                short_units: None,
                short_client_extensions: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Close Position
    /// Closeout the open Position for a specific instrument in an Account.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ClosePositionRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl ClosePositionRequest {
        pub fn new() -> ClosePositionRequest {
            ClosePositionRequest {
                uri: String::from("/v3/accounts/{accountID}/positions/{instrument}/close"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return ClosePositionRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// Name of the Instrument
        /// format: A string containing the base currency and quote currency delimited by
        /// a "_".
        /// - param String
        /// - return ClosePositionRequest
        pub fn with_instrument(mut self, x: String) -> Self {
            self.path.instrument = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return ClosePositionRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return ClosePositionRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// Indication of how much of the long Position to closeout. Either the
        /// string "ALL", the string "NONE", or a DecimalNumber representing how
        /// many units of the long position to close using a PositionCloseout
        /// MarketOrder. The units specified must always be positive.
        /// - param String
        /// - return ClosePositionRequest
        pub fn with_long_units(mut self, x: String) -> Self {
            self.body.long_units = Some(x);
            self
        }

        /// A ClientExtensions object allows a client to attach a clientID, tag
        /// and comment to Orders and Trades in their Account.  Do not set,
        /// modify, or delete this field if your account is associated with MT4.
        /// - param ClientExtensions
        /// - return ClosePositionRequest
        pub fn with_long_client_extensions(mut self, x: ClientExtensions) -> Self {
            self.body.long_client_extensions = Some(x);
            self
        }

        /// Indication of how much of the short Position to closeout. Either the
        /// string "ALL", the string "NONE", or a DecimalNumber representing how
        /// many units of the short position to close using a PositionCloseout
        /// MarketOrder. The units specified must always be positive.
        /// - param String
        /// - return ClosePositionRequest
        pub fn with_short_units(mut self, x: String) -> Self {
            self.body.short_units = Some(x);
            self
        }

        /// A ClientExtensions object allows a client to attach a clientID, tag
        /// and comment to Orders and Trades in their Account.  Do not set,
        /// modify, or delete this field if your account is associated with MT4.
        /// - param ClientExtensions
        /// - return ClosePositionRequest
        pub fn with_short_client_extensions(mut self, x: ClientExtensions) -> Self {
            self.body.short_client_extensions = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<ClosePositionResponse, Box<dyn Error>> {
            let uri = self.uri
                .clone()
                .replace("{accountID}", &self.path.account_id.unwrap())
                .replace("{instrument}", &self.path.instrument.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .put(&url)
                .query(&self.query)
                .json::<RequestBody>(&self.body)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<ClosePositionResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type ClosePositionResponse = ClosePositionResponse200Body;

    /// The Position closeout request has been successfully processed.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ClosePositionResponse200Header {
        /// A link to the Position that was just closed out
        #[serde(rename = "Location", skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The Position closeout request has been successfully processed.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ClosePositionResponse200Body {
        /// A MarketOrderTransaction represents the creation of a Market Order in
        /// the user's account. A Market Order is an Order that is filled
        /// immediately at the current market price. Market Orders can be
        /// specialized when they are created to accomplish a specific task: to
        /// close a Trade, to closeout a Position or to particiate in in a Margin
        /// closeout.
        #[serde(rename = "longOrderCreateTransaction", skip_serializing_if = "Option::is_none")]
        pub long_order_create_transaction: Option<MarketOrderTransaction>,
        /// An OrderFillTransaction represents the filling of an Order in the
        /// client's Account.
        #[serde(rename = "longOrderFillTransaction", skip_serializing_if = "Option::is_none")]
        pub long_order_fill_transaction: Option<OrderFillTransaction>,
        /// An OrderCancelTransaction represents the cancellation of an Order in
        /// the client's Account.
        #[serde(rename = "longOrderCancelTransaction", skip_serializing_if = "Option::is_none")]
        pub long_order_cancel_transaction: Option<OrderCancelTransaction>,
        /// A MarketOrderTransaction represents the creation of a Market Order in
        /// the user's account. A Market Order is an Order that is filled
        /// immediately at the current market price. Market Orders can be
        /// specialized when they are created to accomplish a specific task: to
        /// close a Trade, to closeout a Position or to particiate in in a Margin
        /// closeout.
        #[serde(rename = "shortOrderCreateTransaction", skip_serializing_if = "Option::is_none")]
        pub short_order_create_transaction: Option<MarketOrderTransaction>,
        /// An OrderFillTransaction represents the filling of an Order in the
        /// client's Account.
        #[serde(rename = "shortOrderFillTransaction", skip_serializing_if = "Option::is_none")]
        pub short_order_fill_transaction: Option<OrderFillTransaction>,
        /// An OrderCancelTransaction represents the cancellation of an Order in
        /// the client's Account.
        #[serde(rename = "shortOrderCancelTransaction", skip_serializing_if = "Option::is_none")]
        pub short_order_cancel_transaction: Option<OrderCancelTransaction>,
        /// The IDs of all Transactions that were created while satisfying the
        /// request.
        #[serde(rename = "relatedTransactionIDs", skip_serializing_if = "Option::is_none")]
        pub related_transaction_i_ds: Option<Vec<String>>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod list_trades {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
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
        #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
        pub ids: Option<Vec<String>>,

        #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,

        #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
        pub instrument: Option<String>,

        #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
        pub count: Option<i32>,

        #[serde(rename = "beforeID", skip_serializing_if = "Option::is_none")]
        pub before_id: Option<String>,
    }
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {
                ids: None,
                state: None,
                instrument: None,
                count: None,
                before_id: None,
            }
        }
    }

    /// List Trades
    /// Get a list of Trades for an Account

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListTradesRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl ListTradesRequest {
        pub fn new() -> ListTradesRequest {
            ListTradesRequest {
                uri: String::from("/v3/accounts/{accountID}/trades"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return ListTradesRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return ListTradesRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return ListTradesRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// List of Trade IDs to retrieve.
        /// - param Vec<String>
        /// - return ListTradesRequest
        pub fn with_ids(mut self, x: Vec<String>) -> Self {
            self.query.ids = Some(x);
            self
        }

        /// The state to filter the requested Trades by.
        /// - param String
        /// - return ListTradesRequest
        pub fn with_state(mut self, x: String) -> Self {
            self.query.state = Some(x);
            self
        }

        /// The instrument to filter the requested Trades by.
        /// format: A string containing the base currency and quote currency delimited by
        /// a "_".
        /// - param String
        /// - return ListTradesRequest
        pub fn with_instrument(mut self, x: String) -> Self {
            self.query.instrument = Some(x);
            self
        }

        /// The maximum number of Trades to return.
        /// - param i32
        /// - return ListTradesRequest
        pub fn with_count(mut self, x: i32) -> Self {
            self.query.count = Some(x);
            self
        }

        /// The maximum Trade ID to return. If not provided the most recent Trades
        /// in the Account are returned.
        /// format: The string representation of the OANDA-assigned TradeID. OANDA-
        /// assigned TradeIDs are positive integers, and are derived from the
        /// TransactionID of the Transaction that opened the Trade.
        /// - param String
        /// - return ListTradesRequest
        pub fn with_before_id(mut self, x: String) -> Self {
            self.query.before_id = Some(x);
            self
        }

        pub async fn remote(self, client: &Client) -> Result<ListTradesResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<ListTradesResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type ListTradesResponse = ListTradesResponse200Body;

    /// The list of Trades requested
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListTradesResponse200Header {
        /// A link to the next page of Trades if the results were paginated
        #[serde(rename = "Link", skip_serializing_if = "Option::is_none")]
        pub link: Option<String>,
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The list of Trades requested
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListTradesResponse200Body {
        /// The list of Trade detail objects
        #[serde(rename = "trades", skip_serializing_if = "Option::is_none")]
        pub trades: Option<Vec<Trade>>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod list_open_trades {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
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
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// List Open Trades
    /// Get the list of open Trades for an Account

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListOpenTradesRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl ListOpenTradesRequest {
        pub fn new() -> ListOpenTradesRequest {
            ListOpenTradesRequest {
                uri: String::from("/v3/accounts/{accountID}/openTrades"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return ListOpenTradesRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return ListOpenTradesRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return ListOpenTradesRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<ListOpenTradesResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<ListOpenTradesResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type ListOpenTradesResponse = ListOpenTradesResponse200Body;

    /// The Account's list of open Trades is provided
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListOpenTradesResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The Account's list of open Trades is provided
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListOpenTradesResponse200Body {
        /// The Account's list of open Trades
        #[serde(rename = "trades", skip_serializing_if = "Option::is_none")]
        pub trades: Option<Vec<Trade>>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod get_trade {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        #[serde(rename = "tradeSpecifier", skip_serializing_if = "Option::is_none")]
        pub trade_specifier: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath {
                account_id: None,
                trade_specifier: None,
            }
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
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Trade Details
    /// Get the details of a specific Trade in an Account

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetTradeRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetTradeRequest {
        pub fn new() -> GetTradeRequest {
            GetTradeRequest {
                uri: String::from("/v3/accounts/{accountID}/trades/{tradeSpecifier}"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return GetTradeRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// Specifier for the Trade
        /// format: Either the Trade's OANDA-assigned TradeID or the Trade's client-
        /// provided ClientID prefixed by the "@" symbol
        /// - param String
        /// - return GetTradeRequest
        pub fn with_trade_specifier(mut self, x: String) -> Self {
            self.path.trade_specifier = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetTradeRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return GetTradeRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        pub async fn remote(self, client: &Client) -> Result<GetTradeResponse, Box<dyn Error>> {
            let uri = self.uri
                .clone()
                .replace("{accountID}", &self.path.account_id.unwrap())
                .replace("{tradeSpecifier}", &self.path.trade_specifier.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<GetTradeResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type GetTradeResponse = GetTradeResponse200Body;

    /// The details for the requested Trade is provided
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetTradeResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The details for the requested Trade is provided
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetTradeResponse200Body {
        /// The specification of a Trade within an Account. This includes the full
        /// representation of the Trade's dependent Orders in addition to the IDs
        /// of those Orders.
        #[serde(rename = "trade", skip_serializing_if = "Option::is_none")]
        pub trade: Option<Trade>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod close_trade {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        #[serde(rename = "tradeSpecifier", skip_serializing_if = "Option::is_none")]
        pub trade_specifier: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath {
                account_id: None,
                trade_specifier: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestBody {
        #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
        pub units: Option<String>,
    }
    impl RequestBody {
        fn new() -> RequestBody {
            RequestBody { units: None }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Close Trade
    /// Close (partially or fully) a specific open Trade in an Account

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CloseTradeRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl CloseTradeRequest {
        pub fn new() -> CloseTradeRequest {
            CloseTradeRequest {
                uri: String::from("/v3/accounts/{accountID}/trades/{tradeSpecifier}/close"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return CloseTradeRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// Specifier for the Trade
        /// format: Either the Trade's OANDA-assigned TradeID or the Trade's client-
        /// provided ClientID prefixed by the "@" symbol
        /// - param String
        /// - return CloseTradeRequest
        pub fn with_trade_specifier(mut self, x: String) -> Self {
            self.path.trade_specifier = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return CloseTradeRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return CloseTradeRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// Indication of how much of the Trade to close. Either the string "ALL"
        /// (indicating that all of the Trade should be closed), or a
        /// DecimalNumber representing the number of units of the open Trade to
        /// Close using a TradeClose MarketOrder. The units specified must always
        /// be positive, and the magnitude of the value cannot exceed the
        /// magnitude of the Trade's open units.
        /// - param String
        /// - return CloseTradeRequest
        pub fn with_units(mut self, x: String) -> Self {
            self.body.units = Some(x);
            self
        }

        pub async fn remote(self, client: &Client) -> Result<CloseTradeResponse, Box<dyn Error>> {
            let uri = self.uri
                .clone()
                .replace("{accountID}", &self.path.account_id.unwrap())
                .replace("{tradeSpecifier}", &self.path.trade_specifier.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .put(&url)
                .query(&self.query)
                .json::<RequestBody>(&self.body)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<CloseTradeResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type CloseTradeResponse = CloseTradeResponse200Body;

    /// The Trade has been closed as requested
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CloseTradeResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The Trade has been closed as requested
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CloseTradeResponse200Body {
        /// A MarketOrderTransaction represents the creation of a Market Order in
        /// the user's account. A Market Order is an Order that is filled
        /// immediately at the current market price. Market Orders can be
        /// specialized when they are created to accomplish a specific task: to
        /// close a Trade, to closeout a Position or to particiate in in a Margin
        /// closeout.
        #[serde(rename = "orderCreateTransaction", skip_serializing_if = "Option::is_none")]
        pub order_create_transaction: Option<MarketOrderTransaction>,
        /// An OrderFillTransaction represents the filling of an Order in the
        /// client's Account.
        #[serde(rename = "orderFillTransaction", skip_serializing_if = "Option::is_none")]
        pub order_fill_transaction: Option<OrderFillTransaction>,
        /// An OrderCancelTransaction represents the cancellation of an Order in
        /// the client's Account.
        #[serde(rename = "orderCancelTransaction", skip_serializing_if = "Option::is_none")]
        pub order_cancel_transaction: Option<OrderCancelTransaction>,
        /// The IDs of all Transactions that were created while satisfying the
        /// request.
        #[serde(rename = "relatedTransactionIDs", skip_serializing_if = "Option::is_none")]
        pub related_transaction_i_ds: Option<Vec<String>>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod set_trade_client_extensions {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        #[serde(rename = "tradeSpecifier", skip_serializing_if = "Option::is_none")]
        pub trade_specifier: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath {
                account_id: None,
                trade_specifier: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestBody {
        #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
        pub client_extensions: Option<ClientExtensions>,
    }
    impl RequestBody {
        fn new() -> RequestBody {
            RequestBody {
                client_extensions: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Set Trade Client Extensions
    /// Update the Client Extensions for a Trade. Do not add, update, or
    /// delete the Client Extensions if your account is associated with MT4.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetTradeClientExtensionsRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl SetTradeClientExtensionsRequest {
        pub fn new() -> SetTradeClientExtensionsRequest {
            SetTradeClientExtensionsRequest {
                uri: String::from(
                    "/v3/accounts/{accountID}/trades/{tradeSpecifier}/clientExtensions"
                ),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return SetTradeClientExtensionsRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// Specifier for the Trade
        /// format: Either the Trade's OANDA-assigned TradeID or the Trade's client-
        /// provided ClientID prefixed by the "@" symbol
        /// - param String
        /// - return SetTradeClientExtensionsRequest
        pub fn with_trade_specifier(mut self, x: String) -> Self {
            self.path.trade_specifier = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return SetTradeClientExtensionsRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return SetTradeClientExtensionsRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// A ClientExtensions object allows a client to attach a clientID, tag
        /// and comment to Orders and Trades in their Account.  Do not set,
        /// modify, or delete this field if your account is associated with MT4.
        /// - param ClientExtensions
        /// - return SetTradeClientExtensionsRequest
        pub fn with_client_extensions(mut self, x: ClientExtensions) -> Self {
            self.body.client_extensions = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<SetTradeClientExtensionsResponse, Box<dyn Error>> {
            let uri = self.uri
                .clone()
                .replace("{accountID}", &self.path.account_id.unwrap())
                .replace("{tradeSpecifier}", &self.path.trade_specifier.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .put(&url)
                .query(&self.query)
                .json::<RequestBody>(&self.body)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<SetTradeClientExtensionsResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type SetTradeClientExtensionsResponse = SetTradeClientExtensionsResponse200Body;

    /// The Trade's Client Extensions have been updated as requested.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetTradeClientExtensionsResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The Trade's Client Extensions have been updated as requested.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetTradeClientExtensionsResponse200Body {
        /// A TradeClientExtensionsModifyTransaction represents the modification
        /// of a Trade's Client Extensions.
        #[serde(
            rename = "tradeClientExtensionsModifyTransaction",
            skip_serializing_if = "Option::is_none"
        )]
        pub trade_client_extensions_modify_transaction: Option<TradeClientExtensionsModifyTransaction>,
        /// The IDs of all Transactions that were created while satisfying the
        /// request.
        #[serde(rename = "relatedTransactionIDs", skip_serializing_if = "Option::is_none")]
        pub related_transaction_i_ds: Option<Vec<String>>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod set_trade_dependent_orders {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        #[serde(rename = "tradeSpecifier", skip_serializing_if = "Option::is_none")]
        pub trade_specifier: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath {
                account_id: None,
                trade_specifier: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestBody {
        #[serde(rename = "takeProfit", skip_serializing_if = "Option::is_none")]
        pub take_profit: Option<TakeProfitDetails>,

        #[serde(rename = "stopLoss", skip_serializing_if = "Option::is_none")]
        pub stop_loss: Option<StopLossDetails>,

        #[serde(rename = "trailingStopLoss", skip_serializing_if = "Option::is_none")]
        pub trailing_stop_loss: Option<TrailingStopLossDetails>,
    }
    impl RequestBody {
        fn new() -> RequestBody {
            RequestBody {
                take_profit: None,
                stop_loss: None,
                trailing_stop_loss: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Set Dependent Orders
    /// Create, replace and cancel a Trade's dependent Orders (Take Profit,
    /// Stop Loss and Trailing Stop Loss) through the Trade itself

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetTradeDependentOrdersRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl SetTradeDependentOrdersRequest {
        pub fn new() -> SetTradeDependentOrdersRequest {
            SetTradeDependentOrdersRequest {
                uri: String::from("/v3/accounts/{accountID}/trades/{tradeSpecifier}/orders"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return SetTradeDependentOrdersRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// Specifier for the Trade
        /// format: Either the Trade's OANDA-assigned TradeID or the Trade's client-
        /// provided ClientID prefixed by the "@" symbol
        /// - param String
        /// - return SetTradeDependentOrdersRequest
        pub fn with_trade_specifier(mut self, x: String) -> Self {
            self.path.trade_specifier = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return SetTradeDependentOrdersRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return SetTradeDependentOrdersRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// TakeProfitDetails specifies the details of a Take Profit Order to be
        /// created on behalf of a client. This may happen when an Order is filled
        /// that opens a Trade requiring a Take Profit, or when a Trade's
        /// dependent Take Profit Order is modified directly through the Trade.
        /// - param TakeProfitDetails
        /// - return SetTradeDependentOrdersRequest
        pub fn with_take_profit(mut self, x: TakeProfitDetails) -> Self {
            self.body.take_profit = Some(x);
            self
        }

        /// StopLossDetails specifies the details of a Stop Loss Order to be
        /// created on behalf of a client. This may happen when an Order is filled
        /// that opens a Trade requiring a Stop Loss, or when a Trade's dependent
        /// Stop Loss Order is modified directly through the Trade.
        /// - param StopLossDetails
        /// - return SetTradeDependentOrdersRequest
        pub fn with_stop_loss(mut self, x: StopLossDetails) -> Self {
            self.body.stop_loss = Some(x);
            self
        }

        /// TrailingStopLossDetails specifies the details of a Trailing Stop Loss
        /// Order to be created on behalf of a client. This may happen when an
        /// Order is filled that opens a Trade requiring a Trailing Stop Loss, or
        /// when a Trade's dependent Trailing Stop Loss Order is modified directly
        /// through the Trade.
        /// - param TrailingStopLossDetails
        /// - return SetTradeDependentOrdersRequest
        pub fn with_trailing_stop_loss(mut self, x: TrailingStopLossDetails) -> Self {
            self.body.trailing_stop_loss = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<SetTradeDependentOrdersResponse, Box<dyn Error>> {
            let uri = self.uri
                .clone()
                .replace("{accountID}", &self.path.account_id.unwrap())
                .replace("{tradeSpecifier}", &self.path.trade_specifier.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .put(&url)
                .query(&self.query)
                .json::<RequestBody>(&self.body)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<SetTradeDependentOrdersResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type SetTradeDependentOrdersResponse = SetTradeDependentOrdersResponse200Body;

    /// The Trade's dependent Orders have been modified as requested.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetTradeDependentOrdersResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The Trade's dependent Orders have been modified as requested.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetTradeDependentOrdersResponse200Body {
        /// An OrderCancelTransaction represents the cancellation of an Order in
        /// the client's Account.
        #[serde(
            rename = "takeProfitOrderCancelTransaction",
            skip_serializing_if = "Option::is_none"
        )]
        pub take_profit_order_cancel_transaction: Option<OrderCancelTransaction>,
        /// A TakeProfitOrderTransaction represents the creation of a TakeProfit
        /// Order in the user's Account.
        #[serde(rename = "takeProfitOrderTransaction", skip_serializing_if = "Option::is_none")]
        pub take_profit_order_transaction: Option<TakeProfitOrderTransaction>,
        /// An OrderFillTransaction represents the filling of an Order in the
        /// client's Account.
        #[serde(rename = "takeProfitOrderFillTransaction", skip_serializing_if = "Option::is_none")]
        pub take_profit_order_fill_transaction: Option<OrderFillTransaction>,
        /// An OrderCancelTransaction represents the cancellation of an Order in
        /// the client's Account.
        #[serde(
            rename = "takeProfitOrderCreatedCancelTransaction",
            skip_serializing_if = "Option::is_none"
        )]
        pub take_profit_order_created_cancel_transaction: Option<OrderCancelTransaction>,
        /// An OrderCancelTransaction represents the cancellation of an Order in
        /// the client's Account.
        #[serde(rename = "stopLossOrderCancelTransaction", skip_serializing_if = "Option::is_none")]
        pub stop_loss_order_cancel_transaction: Option<OrderCancelTransaction>,
        /// A StopLossOrderTransaction represents the creation of a StopLoss Order
        /// in the user's Account.
        #[serde(rename = "stopLossOrderTransaction", skip_serializing_if = "Option::is_none")]
        pub stop_loss_order_transaction: Option<StopLossOrderTransaction>,
        /// An OrderFillTransaction represents the filling of an Order in the
        /// client's Account.
        #[serde(rename = "stopLossOrderFillTransaction", skip_serializing_if = "Option::is_none")]
        pub stop_loss_order_fill_transaction: Option<OrderFillTransaction>,
        /// An OrderCancelTransaction represents the cancellation of an Order in
        /// the client's Account.
        #[serde(
            rename = "stopLossOrderCreatedCancelTransaction",
            skip_serializing_if = "Option::is_none"
        )]
        pub stop_loss_order_created_cancel_transaction: Option<OrderCancelTransaction>,
        /// An OrderCancelTransaction represents the cancellation of an Order in
        /// the client's Account.
        #[serde(
            rename = "trailingStopLossOrderCancelTransaction",
            skip_serializing_if = "Option::is_none"
        )]
        pub trailing_stop_loss_order_cancel_transaction: Option<OrderCancelTransaction>,
        /// A TrailingStopLossOrderTransaction represents the creation of a
        /// TrailingStopLoss Order in the user's Account.
        #[serde(
            rename = "trailingStopLossOrderTransaction",
            skip_serializing_if = "Option::is_none"
        )]
        pub trailing_stop_loss_order_transaction: Option<TrailingStopLossOrderTransaction>,
        /// The IDs of all Transactions that were created while satisfying the
        /// request.
        #[serde(rename = "relatedTransactionIDs", skip_serializing_if = "Option::is_none")]
        pub related_transaction_i_ds: Option<Vec<String>>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod list_accounts {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,
    }
    impl RequestHead {
        fn new() -> RequestHead {
            RequestHead {
                authorization: None,
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
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// List Accounts
    /// Get a list of all Accounts authorized for the provided token.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListAccountsRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl ListAccountsRequest {
        pub fn new() -> ListAccountsRequest {
            ListAccountsRequest {
                uri: String::from("/v3/accounts"),
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
        /// - return ListAccountsRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        pub async fn remote(self, client: &Client) -> Result<ListAccountsResponse, Box<dyn Error>> {
            let uri = self.uri.clone();
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<ListAccountsResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type ListAccountsResponse = ListAccountsResponse200Body;

    /// The list of authorized Accounts has been provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListAccountsResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The list of authorized Accounts has been provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListAccountsResponse200Body {
        /// The list of Accounts the client is authorized to access and their
        /// associated properties.
        #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
        pub accounts: Option<Vec<AccountProperties>>,
    }
}

pub mod get_account {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
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
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Account Details
    /// Get the full details for a single Account that a client has access to.
    /// Full pending Order, open Trade and open Position representations are
    /// provided.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetAccountRequest {
        pub fn new() -> GetAccountRequest {
            GetAccountRequest {
                uri: String::from("/v3/accounts/{accountID}"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return GetAccountRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetAccountRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return GetAccountRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        pub async fn remote(self, client: &Client) -> Result<GetAccountResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<GetAccountResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type GetAccountResponse = GetAccountResponse200Body;

    /// The full Account details are provided
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The full Account details are provided
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountResponse200Body {
        /// The full details of a client's Account. This includes full open Trade,
        /// open Position and pending Order representation.
        #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
        pub account: Option<Account>,
        /// The ID of the most recent Transaction created for the Account.
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod get_account_summary {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
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
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Account Summary
    /// Get a summary for a single Account that a client has access to.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountSummaryRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetAccountSummaryRequest {
        pub fn new() -> GetAccountSummaryRequest {
            GetAccountSummaryRequest {
                uri: String::from("/v3/accounts/{accountID}/summary"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return GetAccountSummaryRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetAccountSummaryRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return GetAccountSummaryRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<GetAccountSummaryResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<GetAccountSummaryResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type GetAccountSummaryResponse = GetAccountSummaryResponse200Body;

    /// The Account summary  are provided
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountSummaryResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The Account summary  are provided
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountSummaryResponse200Body {
        /// A summary representation of a client's Account. The AccountSummary
        /// does not provide to full specification of pending Orders, open Trades
        /// and Positions.
        #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
        pub account: Option<AccountSummary>,
        /// The ID of the most recent Transaction created for the Account.
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod get_account_instruments {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,
    }
    impl RequestHead {
        fn new() -> RequestHead {
            RequestHead {
                authorization: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestPath {
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
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
        #[serde(rename = "instruments", skip_serializing_if = "Option::is_none")]
        pub instruments: Option<Vec<String>>,
    }
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery { instruments: None }
        }
    }

    /// Account Instruments
    /// Get the list of tradeable instruments for the given Account. The list
    /// of tradeable instruments is dependent on the regulatory division that
    /// the Account is located in, thus should be the same for all Accounts
    /// owned by a single user.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountInstrumentsRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetAccountInstrumentsRequest {
        pub fn new() -> GetAccountInstrumentsRequest {
            GetAccountInstrumentsRequest {
                uri: String::from("/v3/accounts/{accountID}/instruments"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return GetAccountInstrumentsRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetAccountInstrumentsRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// List of instruments to query specifically.
        /// - param Vec<String>
        /// - return GetAccountInstrumentsRequest
        pub fn with_instruments(mut self, x: Vec<String>) -> Self {
            self.query.instruments = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<GetAccountInstrumentsResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<GetAccountInstrumentsResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type GetAccountInstrumentsResponse = GetAccountInstrumentsResponse200Body;

    /// The list of tradeable instruments for the Account has been provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountInstrumentsResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The list of tradeable instruments for the Account has been provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountInstrumentsResponse200Body {
        /// The requested list of instruments.
        #[serde(rename = "instruments", skip_serializing_if = "Option::is_none")]
        pub instruments: Option<Vec<Instrument>>,
        /// The ID of the most recent Transaction created for the Account.
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod configure_account {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestBody {
        #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
        pub alias: Option<String>,

        #[serde(
            rename = "marginRate",
            skip_serializing_if = "Option::is_none",
            with = "fxoanda_serdes::serfloats"
        )]
        pub margin_rate: Option<f32>,
    }
    impl RequestBody {
        fn new() -> RequestBody {
            RequestBody {
                alias: None,
                margin_rate: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Configure Account
    /// Set the client-configurable portions of an Account.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConfigureAccountRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl ConfigureAccountRequest {
        pub fn new() -> ConfigureAccountRequest {
            ConfigureAccountRequest {
                uri: String::from("/v3/accounts/{accountID}/configuration"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return ConfigureAccountRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return ConfigureAccountRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return ConfigureAccountRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// Client-defined alias (name) for the Account
        /// - param String
        /// - return ConfigureAccountRequest
        pub fn with_alias(mut self, x: String) -> Self {
            self.body.alias = Some(x);
            self
        }

        /// The string representation of a decimal number.
        /// format: A decimal number encoded as a string. The amount of precision provided
        /// depends on what the number represents.
        /// - param f32
        /// - return ConfigureAccountRequest
        pub fn with_margin_rate(mut self, x: f32) -> Self {
            self.body.margin_rate = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<ConfigureAccountResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .patch(&url)
                .query(&self.query)
                .json::<RequestBody>(&self.body)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<ConfigureAccountResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type ConfigureAccountResponse = ConfigureAccountResponse200Body;

    /// The Account was configured successfully.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConfigureAccountResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The Account was configured successfully.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConfigureAccountResponse200Body {
        /// A ClientConfigureTransaction represents the configuration of an
        /// Account by a client.
        #[serde(rename = "clientConfigureTransaction", skip_serializing_if = "Option::is_none")]
        pub client_configure_transaction: Option<ClientConfigureTransaction>,
        /// The ID of the last Transaction created for the Account.
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod get_account_changes {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
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
        #[serde(rename = "sinceTransactionID", skip_serializing_if = "Option::is_none")]
        pub since_transaction_id: Option<String>,
    }
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {
                since_transaction_id: None,
            }
        }
    }

    /// Poll Account Updates
    /// Endpoint used to poll an Account for its current state and changes
    /// since a specified TransactionID.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountChangesRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetAccountChangesRequest {
        pub fn new() -> GetAccountChangesRequest {
            GetAccountChangesRequest {
                uri: String::from("/v3/accounts/{accountID}/changes"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return GetAccountChangesRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetAccountChangesRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return GetAccountChangesRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// ID of the Transaction to get Account changes since.
        /// format: String representation of the numerical OANDA-assigned TransactionID
        /// - param String
        /// - return GetAccountChangesRequest
        pub fn with_since_transaction_id(mut self, x: String) -> Self {
            self.query.since_transaction_id = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<GetAccountChangesResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<GetAccountChangesResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type GetAccountChangesResponse = GetAccountChangesResponse200Body;

    /// The Account state and changes are provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountChangesResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The Account state and changes are provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountChangesResponse200Body {
        /// An AccountChanges Object is used to represent the changes to an
        /// Account's Orders, Trades and Positions since a specified Account
        /// TransactionID in the past.
        #[serde(rename = "changes", skip_serializing_if = "Option::is_none")]
        pub changes: Option<AccountChanges>,
        /// An AccountState Object is used to represent an Account's current
        /// price-dependent state. Price-dependent Account state is dependent on
        /// OANDA's current Prices, and includes things like unrealized PL, NAV
        /// and Trailing Stop Loss Order state.
        #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
        pub state: Option<AccountChangesState>,
        /// The ID of the last Transaction created for the Account.  This
        /// Transaction ID should be used for future poll requests, as the client
        /// has already observed all changes up to and including it.
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod list_transactions {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
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
            with = "fxoanda_serdes::serdates"
        )]
        pub from: Option<DateTime<Utc>>,

        #[serde(
            rename = "to",
            skip_serializing_if = "Option::is_none",
            with = "fxoanda_serdes::serdates"
        )]
        pub to: Option<DateTime<Utc>>,

        #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
        pub page_size: Option<i32>,

        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        pub otype: Option<Vec<String>>,
    }
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {
                from: None,
                to: None,
                page_size: None,
                otype: None,
            }
        }
    }

    /// List Transactions
    /// Get a list of Transactions pages that satisfy a time-based Transaction
    /// query.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListTransactionsRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl ListTransactionsRequest {
        pub fn new() -> ListTransactionsRequest {
            ListTransactionsRequest {
                uri: String::from("/v3/accounts/{accountID}/transactions"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return ListTransactionsRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return ListTransactionsRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return ListTransactionsRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// The starting time (inclusive) of the time range for the Transactions
        /// being queried.
        /// format: The RFC 3339 representation is a string conforming to
        /// https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a
        /// string representing the number of seconds since the Unix Epoch
        /// (January 1st, 1970 at UTC). The value is a fractional number, where
        /// the fractional part represents a fraction of a second (up to nine
        /// decimal places).
        /// - param DateTime<Utc>
        /// - return ListTransactionsRequest
        pub fn with_from(mut self, x: DateTime<Utc>) -> Self {
            self.query.from = Some(x);
            self
        }

        /// The ending time (inclusive) of the time range for the Transactions
        /// being queried.
        /// format: The RFC 3339 representation is a string conforming to
        /// https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a
        /// string representing the number of seconds since the Unix Epoch
        /// (January 1st, 1970 at UTC). The value is a fractional number, where
        /// the fractional part represents a fraction of a second (up to nine
        /// decimal places).
        /// - param DateTime<Utc>
        /// - return ListTransactionsRequest
        pub fn with_to(mut self, x: DateTime<Utc>) -> Self {
            self.query.to = Some(x);
            self
        }

        /// The number of Transactions to include in each page of the results.
        /// - param i32
        /// - return ListTransactionsRequest
        pub fn with_page_size(mut self, x: i32) -> Self {
            self.query.page_size = Some(x);
            self
        }

        /// A filter for restricting the types of Transactions to retreive.
        /// - param Vec<String>
        /// - return ListTransactionsRequest
        pub fn with_otype(mut self, x: Vec<String>) -> Self {
            self.query.otype = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<ListTransactionsResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<ListTransactionsResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type ListTransactionsResponse = ListTransactionsResponse200Body;

    /// The requested time range of Transaction pages are provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListTransactionsResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The requested time range of Transaction pages are provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListTransactionsResponse200Body {
        /// The starting time provided in the request.
        /// format: The RFC 3339 representation is a string conforming to
        /// https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a
        /// string representing the number of seconds since the Unix Epoch
        /// (January 1st, 1970 at UTC). The value is a fractional number, where
        /// the fractional part represents a fraction of a second (up to nine
        /// decimal places).
        #[serde(
            rename = "from",
            skip_serializing_if = "Option::is_none",
            with = "fxoanda_serdes::serdates"
        )]
        pub from: Option<DateTime<Utc>>,
        /// The ending time provided in the request.
        /// format: The RFC 3339 representation is a string conforming to
        /// https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a
        /// string representing the number of seconds since the Unix Epoch
        /// (January 1st, 1970 at UTC). The value is a fractional number, where
        /// the fractional part represents a fraction of a second (up to nine
        /// decimal places).
        #[serde(
            rename = "to",
            skip_serializing_if = "Option::is_none",
            with = "fxoanda_serdes::serdates"
        )]
        pub to: Option<DateTime<Utc>>,
        /// The pageSize provided in the request
        #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
        pub page_size: Option<i32>,
        /// The Transaction-type filter provided in the request
        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        pub otype: Option<Vec<String>>,
        /// The number of Transactions that are contained in the pages returned
        #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
        pub count: Option<i32>,
        /// The list of URLs that represent idrange queries providing the data for
        /// each page in the query results
        #[serde(rename = "pages", skip_serializing_if = "Option::is_none")]
        pub pages: Option<Vec<String>>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod get_transaction {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        #[serde(rename = "transactionID", skip_serializing_if = "Option::is_none")]
        pub transaction_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath {
                account_id: None,
                transaction_id: None,
            }
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
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Transaction Details
    /// Get the details of a single Account Transaction.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetTransactionRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetTransactionRequest {
        pub fn new() -> GetTransactionRequest {
            GetTransactionRequest {
                uri: String::from("/v3/accounts/{accountID}/transactions/{transactionID}"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return GetTransactionRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// A Transaction ID
        /// format: String representation of the numerical OANDA-assigned TransactionID
        /// - param String
        /// - return GetTransactionRequest
        pub fn with_transaction_id(mut self, x: String) -> Self {
            self.path.transaction_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetTransactionRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return GetTransactionRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<GetTransactionResponse, Box<dyn Error>> {
            let uri = self.uri
                .clone()
                .replace("{accountID}", &self.path.account_id.unwrap())
                .replace("{transactionID}", &self.path.transaction_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<GetTransactionResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type GetTransactionResponse = GetTransactionResponse200Body;

    /// The details of the requested Transaction are provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetTransactionResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The details of the requested Transaction are provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetTransactionResponse200Body {
        /// The base Transaction specification. Specifies properties that are
        /// common between all Transaction.
        #[serde(rename = "transaction", skip_serializing_if = "Option::is_none")]
        pub transaction: Option<Transaction>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod get_transaction_range {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
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
        #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
        pub from: Option<String>,

        #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
        pub to: Option<String>,

        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        pub otype: Option<Vec<String>>,
    }
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {
                from: None,
                to: None,
                otype: None,
            }
        }
    }

    /// Transaction ID Range
    /// Get a range of Transactions for an Account based on the Transaction
    /// IDs.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetTransactionRangeRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetTransactionRangeRequest {
        pub fn new() -> GetTransactionRangeRequest {
            GetTransactionRangeRequest {
                uri: String::from("/v3/accounts/{accountID}/transactions/idrange"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return GetTransactionRangeRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetTransactionRangeRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return GetTransactionRangeRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// The starting Transacion ID (inclusive) to fetch.
        /// format: String representation of the numerical OANDA-assigned TransactionID
        /// - param String
        /// - return GetTransactionRangeRequest
        pub fn with_from(mut self, x: String) -> Self {
            self.query.from = Some(x);
            self
        }

        /// The ending Transaction ID (inclusive) to fetch.
        /// format: String representation of the numerical OANDA-assigned TransactionID
        /// - param String
        /// - return GetTransactionRangeRequest
        pub fn with_to(mut self, x: String) -> Self {
            self.query.to = Some(x);
            self
        }

        /// The filter that restricts the types of Transactions to retreive.
        /// - param Vec<String>
        /// - return GetTransactionRangeRequest
        pub fn with_otype(mut self, x: Vec<String>) -> Self {
            self.query.otype = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<GetTransactionRangeResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<GetTransactionRangeResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type GetTransactionRangeResponse = GetTransactionRangeResponse200Body;

    /// The requested time range of Transactions are provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetTransactionRangeResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The requested time range of Transactions are provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetTransactionRangeResponse200Body {
        /// The list of Transactions that satisfy the request.
        #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
        pub transactions: Option<Vec<Transaction>>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod get_transactions_since_id {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
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
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery { id: None }
        }
    }

    /// Transactions Since ID
    /// Get a range of Transactions for an Account starting at (but not
    /// including) a provided Transaction ID.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetTransactionsSinceIdRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetTransactionsSinceIdRequest {
        pub fn new() -> GetTransactionsSinceIdRequest {
            GetTransactionsSinceIdRequest {
                uri: String::from("/v3/accounts/{accountID}/transactions/sinceid"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return GetTransactionsSinceIdRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetTransactionsSinceIdRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return GetTransactionsSinceIdRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// The ID of the last Transacion fetched. This query will return all
        /// Transactions newer than the TransactionID.
        /// format: String representation of the numerical OANDA-assigned TransactionID
        /// - param String
        /// - return GetTransactionsSinceIdRequest
        pub fn with_id(mut self, x: String) -> Self {
            self.query.id = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<GetTransactionsSinceIdResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<GetTransactionsSinceIdResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type GetTransactionsSinceIdResponse = GetTransactionsSinceIdResponse200Body;

    /// The requested time range of Transactions are provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetTransactionsSinceIdResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The requested time range of Transactions are provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetTransactionsSinceIdResponse200Body {
        /// The list of Transactions that satisfy the request.
        #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
        pub transactions: Option<Vec<Transaction>>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod stream_transactions {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,
    }
    impl RequestHead {
        fn new() -> RequestHead {
            RequestHead {
                authorization: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestPath {
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
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
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Transaction Stream
    /// Get a stream of Transactions for an Account starting from when the
    /// request is made.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct StreamTransactionsRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl StreamTransactionsRequest {
        pub fn new() -> StreamTransactionsRequest {
            StreamTransactionsRequest {
                uri: String::from("/v3/accounts/{accountID}/transactions/stream"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return StreamTransactionsRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return StreamTransactionsRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<StreamTransactionsResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<StreamTransactionsResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type StreamTransactionsResponse = StreamTransactionsResponse200Body;

    /// Connecting to the Transaction Stream was successful.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StreamTransactionsResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// Connecting to the Transaction Stream was successful.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StreamTransactionsResponse200Body {
        /// The base Transaction specification. Specifies properties that are
        /// common between all Transaction.
        #[serde(rename = "transaction", skip_serializing_if = "Option::is_none")]
        pub transaction: Option<Transaction>,
        /// A TransactionHeartbeat object is injected into the Transaction stream
        /// to ensure that the HTTP connection remains active.
        #[serde(rename = "heartbeat", skip_serializing_if = "Option::is_none")]
        pub heartbeat: Option<TransactionHeartbeat>,
    }
}

pub mod get_prices {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
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
        #[serde(rename = "instruments", skip_serializing_if = "Option::is_none")]
        pub instruments: Option<String>,

        #[serde(
            rename = "since",
            skip_serializing_if = "Option::is_none",
            with = "fxoanda_serdes::serdates"
        )]
        pub since: Option<DateTime<Utc>>,

        #[serde(rename = "includeUnitsAvailable", skip_serializing_if = "Option::is_none")]
        pub include_units_available: Option<bool>,

        #[serde(rename = "includeHomeConversions", skip_serializing_if = "Option::is_none")]
        pub include_home_conversions: Option<bool>,
    }
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {
                instruments: None,
                since: None,
                include_units_available: None,
                include_home_conversions: None,
            }
        }
    }

    /// Current Account Prices
    /// Get pricing information for a specified list of Instruments within an
    /// Account.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetPricesRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetPricesRequest {
        pub fn new() -> GetPricesRequest {
            GetPricesRequest {
                uri: String::from("/v3/accounts/{accountID}/pricing"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return GetPricesRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetPricesRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return GetPricesRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// List of Instruments to get pricing for.
        /// - param String
        /// - return GetPricesRequest
        pub fn with_instruments(mut self, x: String) -> Self {
            self.query.instruments = Some(x);
            self
        }

        /// Date/Time filter to apply to the response. Only prices and home
        /// conversions (if requested) with a time later than this filter (i.e.
        /// the price has changed after the since time) will be provided, and are
        /// filtered independently.
        /// format: The RFC 3339 representation is a string conforming to
        /// https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a
        /// string representing the number of seconds since the Unix Epoch
        /// (January 1st, 1970 at UTC). The value is a fractional number, where
        /// the fractional part represents a fraction of a second (up to nine
        /// decimal places).
        /// - param DateTime<Utc>
        /// - return GetPricesRequest
        pub fn with_since(mut self, x: DateTime<Utc>) -> Self {
            self.query.since = Some(x);
            self
        }

        /// Flag that enables the inclusion of the unitsAvailable field in the
        /// returned Price objects.
        /// - param bool
        /// - return GetPricesRequest
        pub fn with_include_units_available(mut self, x: bool) -> Self {
            self.query.include_units_available = Some(x);
            self
        }

        /// Flag that enables the inclusion of the homeConversions field in the
        /// returned response. An entry will be returned for each currency in the
        /// set of all base and quote currencies present in the requested
        /// instruments list.
        /// - param bool
        /// - return GetPricesRequest
        pub fn with_include_home_conversions(mut self, x: bool) -> Self {
            self.query.include_home_conversions = Some(x);
            self
        }

        pub async fn remote(self, client: &Client) -> Result<GetPricesResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<GetPricesResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type GetPricesResponse = GetPricesResponse200Body;

    /// Pricing information has been successfully provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetPricesResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// Pricing information has been successfully provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetPricesResponse200Body {
        /// The list of Price objects requested.
        #[serde(rename = "prices", skip_serializing_if = "Option::is_none")]
        pub prices: Option<Vec<ClientPrice>>,
        /// The list of home currency conversion factors requested. This field
        /// will only be present if includeHomeConversions was set to true in the
        /// request.
        #[serde(rename = "homeConversions", skip_serializing_if = "Option::is_none")]
        pub home_conversions: Option<Vec<HomeConversions>>,
        /// The DateTime value to use for the "since" parameter in the next poll
        /// request.
        /// format: The RFC 3339 representation is a string conforming to
        /// https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a
        /// string representing the number of seconds since the Unix Epoch
        /// (January 1st, 1970 at UTC). The value is a fractional number, where
        /// the fractional part represents a fraction of a second (up to nine
        /// decimal places).
        #[serde(
            rename = "time",
            skip_serializing_if = "Option::is_none",
            with = "fxoanda_serdes::serdates"
        )]
        pub time: Option<DateTime<Utc>>,
    }
}

pub mod stream_pricing {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
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
        #[serde(rename = "instruments", skip_serializing_if = "Option::is_none")]
        pub instruments: Option<Vec<String>>,

        #[serde(rename = "snapshot", skip_serializing_if = "Option::is_none")]
        pub snapshot: Option<bool>,
    }
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {
                instruments: None,
                snapshot: None,
            }
        }
    }

    /// Price Stream
    /// Get a stream of Account Prices starting from when the request is made.
    /// This pricing stream does not include every single price created for
    /// the Account, but instead will provide at most 4 prices per second
    /// (every 250 milliseconds) for each instrument being requested. If more
    /// than one price is created for an instrument during the 250 millisecond
    /// window, only the price in effect at the end of the window is sent.
    /// This means that during periods of rapid price movement, subscribers to
    /// this stream will not be sent every price. Pricing windows for
    /// different connections to the price stream are not all aligned in the
    /// same way (i.e. they are not all aligned to the top of the second).
    /// This means that during periods of rapid price movement, different
    /// subscribers may observe different prices depending on their alignment.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct StreamPricingRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl StreamPricingRequest {
        pub fn new() -> StreamPricingRequest {
            StreamPricingRequest {
                uri: String::from("/v3/accounts/{accountID}/pricing/stream"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return StreamPricingRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return StreamPricingRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return StreamPricingRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// List of Instruments to stream Prices for.
        /// - param Vec<String>
        /// - return StreamPricingRequest
        pub fn with_instruments(mut self, x: Vec<String>) -> Self {
            self.query.instruments = Some(x);
            self
        }

        /// Flag that enables/disables the sending of a pricing snapshot when
        /// initially connecting to the stream.
        /// - param bool
        /// - return StreamPricingRequest
        pub fn with_snapshot(mut self, x: bool) -> Self {
            self.query.snapshot = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<StreamPricingResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<StreamPricingResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type StreamPricingResponse = StreamPricingResponse200Body;

    /// Connecting to the Price Stream was successful.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StreamPricingResponse200Header {
        /// A link to the next/previous order book snapshot.
        #[serde(rename = "Link", skip_serializing_if = "Option::is_none")]
        pub link: Option<String>,
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// Connecting to the Price Stream was successful.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StreamPricingResponse200Body {
        /// The specification of an Account-specific Price.
        #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
        pub price: Option<ClientPrice>,
        /// A PricingHeartbeat object is injected into the Pricing stream to
        /// ensure that the HTTP connection remains active.
        #[serde(rename = "heartbeat", skip_serializing_if = "Option::is_none")]
        pub heartbeat: Option<PricingHeartbeat>,
    }
}

pub mod get_account_instrument_candles {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
            with = "fxoanda_serdes::serdates"
        )]
        pub from: Option<DateTime<Utc>>,

        #[serde(
            rename = "to",
            skip_serializing_if = "Option::is_none",
            with = "fxoanda_serdes::serdates"
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

        #[serde(
            rename = "units",
            skip_serializing_if = "Option::is_none",
            with = "fxoanda_serdes::serfloats"
        )]
        pub units: Option<f32>,
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
                units: None,
            }
        }
    }

    /// Get Candlesticks
    /// Fetch candlestick data for an instrument.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountInstrumentCandlesRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetAccountInstrumentCandlesRequest {
        pub fn new() -> GetAccountInstrumentCandlesRequest {
            GetAccountInstrumentCandlesRequest {
                uri: String::from("/v3/accounts/{accountID}/instruments/{instrument}/candles"),
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
        /// - return GetAccountInstrumentCandlesRequest
        pub fn with_instrument(mut self, x: String) -> Self {
            self.path.instrument = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetAccountInstrumentCandlesRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return GetAccountInstrumentCandlesRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// The Price component(s) to get candlestick data for. Can contain any
        /// combination of the characters "M" (midpoint candles) "B" (bid candles)
        /// and "A" (ask candles).
        /// - param String
        /// - return GetAccountInstrumentCandlesRequest
        pub fn with_price(mut self, x: String) -> Self {
            self.query.price = Some(x);
            self
        }

        /// The granularity of the candlesticks to fetch
        /// - param CandlestickGranularity
        /// - return GetAccountInstrumentCandlesRequest
        pub fn with_granularity(mut self, x: CandlestickGranularity) -> Self {
            self.query.granularity = Some(x);
            self
        }

        /// The number of candlesticks to return in the response. Count should not
        /// be specified if both the start and end parameters are provided, as the
        /// time range combined with the granularity will determine the number of
        /// candlesticks to return.
        /// - param i32
        /// - return GetAccountInstrumentCandlesRequest
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
        /// - return GetAccountInstrumentCandlesRequest
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
        /// - return GetAccountInstrumentCandlesRequest
        pub fn with_to(mut self, x: DateTime<Utc>) -> Self {
            self.query.to = Some(x);
            self
        }

        /// A flag that controls whether the candlestick is "smoothed" or not.  A
        /// smoothed candlestick uses the previous candle's close price as its
        /// open price, while an unsmoothed candlestick uses the first price from
        /// its time range as its open price.
        /// - param bool
        /// - return GetAccountInstrumentCandlesRequest
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
        /// - return GetAccountInstrumentCandlesRequest
        pub fn with_include_first(mut self, x: bool) -> Self {
            self.query.include_first = Some(x);
            self
        }

        /// The hour of the day (in the specified timezone) to use for
        /// granularities that have daily alignments.
        /// - param i32
        /// - return GetAccountInstrumentCandlesRequest
        pub fn with_daily_alignment(mut self, x: i32) -> Self {
            self.query.daily_alignment = Some(x);
            self
        }

        /// The timezone to use for the dailyAlignment parameter. Candlesticks
        /// with daily alignment will be aligned to the dailyAlignment hour within
        /// the alignmentTimezone.  Note that the returned times will still be
        /// represented in UTC.
        /// - param String
        /// - return GetAccountInstrumentCandlesRequest
        pub fn with_alignment_timezone(mut self, x: String) -> Self {
            self.query.alignment_timezone = Some(x);
            self
        }

        /// The day of the week used for granularities that have weekly alignment.
        /// - param String
        /// - return GetAccountInstrumentCandlesRequest
        pub fn with_weekly_alignment(mut self, x: String) -> Self {
            self.query.weekly_alignment = Some(x);
            self
        }

        /// The number of units used to calculate the volume-weighted average bid
        /// and ask prices in the returned candles.
        /// format: A decimal number encoded as a string. The amount of precision provided
        /// depends on what the number represents.
        /// - param f32
        /// - return GetAccountInstrumentCandlesRequest
        pub fn with_units(mut self, x: f32) -> Self {
            self.query.units = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<GetAccountInstrumentCandlesResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{instrument}", &self.path.instrument.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<GetAccountInstrumentCandlesResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type GetAccountInstrumentCandlesResponse = GetAccountInstrumentCandlesResponse200Body;

    /// Pricing information has been successfully provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountInstrumentCandlesResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// Pricing information has been successfully provided.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetAccountInstrumentCandlesResponse200Body {
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

pub mod create_market_order {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestBody {
        #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
        pub order: Option<MarketOrder>,
    }
    impl RequestBody {
        fn new() -> RequestBody {
            RequestBody { order: None }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Create Order
    /// Create an Order for an Account

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateMarketOrderRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl CreateMarketOrderRequest {
        pub fn new() -> CreateMarketOrderRequest {
            CreateMarketOrderRequest {
                uri: String::from("/v3/accounts/{accountID}/orders"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return CreateMarketOrderRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return CreateMarketOrderRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return CreateMarketOrderRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// A request for a MarketOrder   /// - param MarketOrder
        /// - return CreateMarketOrderRequest
        pub fn with_order(mut self, x: MarketOrder) -> Self {
            self.body.order = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<CreateMarketOrderResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .post(&url)
                .query(&self.query)
                .json::<RequestBody>(&self.body)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<CreateMarketOrderResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type CreateMarketOrderResponse = CreateMarketOrderResponse200Body;

    /// The Order was created as specified
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateMarketOrderResponse200Header {
        /// A link to the Order that was just created
        #[serde(rename = "Location", skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The Order was created as specified
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateMarketOrderResponse200Body {
        /// The base Transaction specification. Specifies properties that are
        /// common between all Transaction.
        #[serde(rename = "orderCreateTransaction", skip_serializing_if = "Option::is_none")]
        pub order_create_transaction: Option<Transaction>,
        /// An OrderFillTransaction represents the filling of an Order in the
        /// client's Account.
        #[serde(rename = "orderFillTransaction", skip_serializing_if = "Option::is_none")]
        pub order_fill_transaction: Option<OrderFillTransaction>,
        /// An OrderCancelTransaction represents the cancellation of an Order in
        /// the client's Account.
        #[serde(rename = "orderCancelTransaction", skip_serializing_if = "Option::is_none")]
        pub order_cancel_transaction: Option<OrderCancelTransaction>,
        /// The base Transaction specification. Specifies properties that are
        /// common between all Transaction.
        #[serde(rename = "orderReissueTransaction", skip_serializing_if = "Option::is_none")]
        pub order_reissue_transaction: Option<Transaction>,
        /// The base Transaction specification. Specifies properties that are
        /// common between all Transaction.
        #[serde(rename = "orderReissueRejectTransaction", skip_serializing_if = "Option::is_none")]
        pub order_reissue_reject_transaction: Option<Transaction>,
        /// The IDs of all Transactions that were created while satisfying the
        /// request.
        #[serde(rename = "relatedTransactionIDs", skip_serializing_if = "Option::is_none")]
        pub related_transaction_i_ds: Option<Vec<String>>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod create_limit_order {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestBody {
        #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
        pub order: Option<LimitOrder>,
    }
    impl RequestBody {
        fn new() -> RequestBody {
            RequestBody { order: None }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Create Order
    /// Create an Order for an Account

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateLimitOrderRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl CreateLimitOrderRequest {
        pub fn new() -> CreateLimitOrderRequest {
            CreateLimitOrderRequest {
                uri: String::from("/v3/accounts/{accountID}/orders"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return CreateLimitOrderRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return CreateLimitOrderRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return CreateLimitOrderRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// A request for a LimitOrder   /// - param LimitOrder
        /// - return CreateLimitOrderRequest
        pub fn with_order(mut self, x: LimitOrder) -> Self {
            self.body.order = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<CreateLimitOrderResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .post(&url)
                .query(&self.query)
                .json::<RequestBody>(&self.body)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<CreateLimitOrderResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type CreateLimitOrderResponse = CreateLimitOrderResponse200Body;

    /// The Order was created as specified
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateLimitOrderResponse200Header {
        /// A link to the Order that was just created
        #[serde(rename = "Location", skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The Order was created as specified
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateLimitOrderResponse200Body {
        /// The base Transaction specification. Specifies properties that are
        /// common between all Transaction.
        #[serde(rename = "orderCreateTransaction", skip_serializing_if = "Option::is_none")]
        pub order_create_transaction: Option<Transaction>,
        /// An OrderFillTransaction represents the filling of an Order in the
        /// client's Account.
        #[serde(rename = "orderFillTransaction", skip_serializing_if = "Option::is_none")]
        pub order_fill_transaction: Option<OrderFillTransaction>,
        /// An OrderCancelTransaction represents the cancellation of an Order in
        /// the client's Account.
        #[serde(rename = "orderCancelTransaction", skip_serializing_if = "Option::is_none")]
        pub order_cancel_transaction: Option<OrderCancelTransaction>,
        /// The base Transaction specification. Specifies properties that are
        /// common between all Transaction.
        #[serde(rename = "orderReissueTransaction", skip_serializing_if = "Option::is_none")]
        pub order_reissue_transaction: Option<Transaction>,
        /// The base Transaction specification. Specifies properties that are
        /// common between all Transaction.
        #[serde(rename = "orderReissueRejectTransaction", skip_serializing_if = "Option::is_none")]
        pub order_reissue_reject_transaction: Option<Transaction>,
        /// The IDs of all Transactions that were created while satisfying the
        /// request.
        #[serde(rename = "relatedTransactionIDs", skip_serializing_if = "Option::is_none")]
        pub related_transaction_i_ds: Option<Vec<String>>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod create_stop_order {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestBody {
        #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
        pub order: Option<StopOrder>,
    }
    impl RequestBody {
        fn new() -> RequestBody {
            RequestBody { order: None }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Create Order
    /// Create an Order for an Account

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateStopOrderRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl CreateStopOrderRequest {
        pub fn new() -> CreateStopOrderRequest {
            CreateStopOrderRequest {
                uri: String::from("/v3/accounts/{accountID}/orders"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return CreateStopOrderRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return CreateStopOrderRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return CreateStopOrderRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// A request for a StopOrder   /// - param StopOrder
        /// - return CreateStopOrderRequest
        pub fn with_order(mut self, x: StopOrder) -> Self {
            self.body.order = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<CreateStopOrderResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .post(&url)
                .query(&self.query)
                .json::<RequestBody>(&self.body)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<CreateStopOrderResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type CreateStopOrderResponse = CreateStopOrderResponse200Body;

    /// The Order was created as specified
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateStopOrderResponse200Header {
        /// A link to the Order that was just created
        #[serde(rename = "Location", skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The Order was created as specified
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateStopOrderResponse200Body {
        /// The base Transaction specification. Specifies properties that are
        /// common between all Transaction.
        #[serde(rename = "orderCreateTransaction", skip_serializing_if = "Option::is_none")]
        pub order_create_transaction: Option<Transaction>,
        /// An OrderFillTransaction represents the filling of an Order in the
        /// client's Account.
        #[serde(rename = "orderFillTransaction", skip_serializing_if = "Option::is_none")]
        pub order_fill_transaction: Option<OrderFillTransaction>,
        /// An OrderCancelTransaction represents the cancellation of an Order in
        /// the client's Account.
        #[serde(rename = "orderCancelTransaction", skip_serializing_if = "Option::is_none")]
        pub order_cancel_transaction: Option<OrderCancelTransaction>,
        /// The base Transaction specification. Specifies properties that are
        /// common between all Transaction.
        #[serde(rename = "orderReissueTransaction", skip_serializing_if = "Option::is_none")]
        pub order_reissue_transaction: Option<Transaction>,
        /// The base Transaction specification. Specifies properties that are
        /// common between all Transaction.
        #[serde(rename = "orderReissueRejectTransaction", skip_serializing_if = "Option::is_none")]
        pub order_reissue_reject_transaction: Option<Transaction>,
        /// The IDs of all Transactions that were created while satisfying the
        /// request.
        #[serde(rename = "relatedTransactionIDs", skip_serializing_if = "Option::is_none")]
        pub related_transaction_i_ds: Option<Vec<String>>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod list_orders {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
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
        #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
        pub ids: Option<Vec<String>>,

        #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,

        #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
        pub instrument: Option<String>,

        #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
        pub count: Option<i32>,

        #[serde(rename = "beforeID", skip_serializing_if = "Option::is_none")]
        pub before_id: Option<String>,
    }
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {
                ids: None,
                state: None,
                instrument: None,
                count: None,
                before_id: None,
            }
        }
    }

    /// List Orders
    /// Get a list of Orders for an Account

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListOrdersRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl ListOrdersRequest {
        pub fn new() -> ListOrdersRequest {
            ListOrdersRequest {
                uri: String::from("/v3/accounts/{accountID}/orders"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return ListOrdersRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return ListOrdersRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return ListOrdersRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// List of Order IDs to retrieve
        /// - param Vec<String>
        /// - return ListOrdersRequest
        pub fn with_ids(mut self, x: Vec<String>) -> Self {
            self.query.ids = Some(x);
            self
        }

        /// The state to filter the requested Orders by
        /// - param String
        /// - return ListOrdersRequest
        pub fn with_state(mut self, x: String) -> Self {
            self.query.state = Some(x);
            self
        }

        /// The instrument to filter the requested orders by
        /// format: A string containing the base currency and quote currency delimited by
        /// a "_".
        /// - param String
        /// - return ListOrdersRequest
        pub fn with_instrument(mut self, x: String) -> Self {
            self.query.instrument = Some(x);
            self
        }

        /// The maximum number of Orders to return
        /// - param i32
        /// - return ListOrdersRequest
        pub fn with_count(mut self, x: i32) -> Self {
            self.query.count = Some(x);
            self
        }

        /// The maximum Order ID to return. If not provided the most recent Orders
        /// in the Account are returned
        /// format: The string representation of the OANDA-assigned OrderID. OANDA-
        /// assigned OrderIDs are positive integers, and are derived from the
        /// TransactionID of the Transaction that created the Order.
        /// - param String
        /// - return ListOrdersRequest
        pub fn with_before_id(mut self, x: String) -> Self {
            self.query.before_id = Some(x);
            self
        }

        pub async fn remote(self, client: &Client) -> Result<ListOrdersResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<ListOrdersResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type ListOrdersResponse = ListOrdersResponse200Body;

    /// The list of Orders requested
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListOrdersResponse200Header {
        /// A link to the next page of results if the results were paginated
        #[serde(rename = "Link", skip_serializing_if = "Option::is_none")]
        pub link: Option<String>,
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The list of Orders requested
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListOrdersResponse200Body {
        /// The list of Order detail objects
        #[serde(rename = "orders", skip_serializing_if = "Option::is_none")]
        pub orders: Option<Vec<Order>>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod list_pending_orders {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath { account_id: None }
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
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Pending Orders
    /// List all pending Orders in an Account

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListPendingOrdersRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl ListPendingOrdersRequest {
        pub fn new() -> ListPendingOrdersRequest {
            ListPendingOrdersRequest {
                uri: String::from("/v3/accounts/{accountID}/pendingOrders"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return ListPendingOrdersRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return ListPendingOrdersRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return ListPendingOrdersRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<ListPendingOrdersResponse, Box<dyn Error>> {
            let uri = self.uri.clone().replace("{accountID}", &self.path.account_id.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<ListPendingOrdersResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type ListPendingOrdersResponse = ListPendingOrdersResponse200Body;

    /// List of pending Orders for the Account
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListPendingOrdersResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// List of pending Orders for the Account
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ListPendingOrdersResponse200Body {
        /// The list of pending Order details
        #[serde(rename = "orders", skip_serializing_if = "Option::is_none")]
        pub orders: Option<Vec<Order>>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod get_order {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        #[serde(rename = "orderSpecifier", skip_serializing_if = "Option::is_none")]
        pub order_specifier: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath {
                account_id: None,
                order_specifier: None,
            }
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
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Get Order
    /// Get details for a single Order in an Account

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetOrderRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl GetOrderRequest {
        pub fn new() -> GetOrderRequest {
            GetOrderRequest {
                uri: String::from("/v3/accounts/{accountID}/orders/{orderSpecifier}"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return GetOrderRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The Order Specifier
        /// format: Either the Order's OANDA-assigned OrderID or the Order's client-
        /// provided ClientID prefixed by the "@" symbol
        /// - param String
        /// - return GetOrderRequest
        pub fn with_order_specifier(mut self, x: String) -> Self {
            self.path.order_specifier = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return GetOrderRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return GetOrderRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        pub async fn remote(self, client: &Client) -> Result<GetOrderResponse, Box<dyn Error>> {
            let uri = self.uri
                .clone()
                .replace("{accountID}", &self.path.account_id.unwrap())
                .replace("{orderSpecifier}", &self.path.order_specifier.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .get(&url)
                .query(&self.query)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<GetOrderResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type GetOrderResponse = GetOrderResponse200Body;

    /// The details of the Order requested
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetOrderResponse200Header {
        /// A link to the next page of results if the results were paginated
        #[serde(rename = "Link", skip_serializing_if = "Option::is_none")]
        pub link: Option<String>,
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The details of the Order requested
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetOrderResponse200Body {
        /// The base Order definition specifies the properties that are common to
        /// all Orders.
        #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
        pub order: Option<Order>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod replace_order {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
        pub accept_datetime_format: Option<String>,

        #[serde(rename = "ClientRequestID", skip_serializing_if = "Option::is_none")]
        pub client_request_id: Option<String>,
    }
    impl RequestHead {
        fn new() -> RequestHead {
            RequestHead {
                authorization: None,
                accept_datetime_format: None,
                client_request_id: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestPath {
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        #[serde(rename = "orderSpecifier", skip_serializing_if = "Option::is_none")]
        pub order_specifier: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath {
                account_id: None,
                order_specifier: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestBody {
        #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
        pub order: Option<OrderRequest>,
    }
    impl RequestBody {
        fn new() -> RequestBody {
            RequestBody { order: None }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Replace Order
    /// Replace an Order in an Account by simultaneously cancelling it and
    /// creating a replacement Order

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReplaceOrderRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl ReplaceOrderRequest {
        pub fn new() -> ReplaceOrderRequest {
            ReplaceOrderRequest {
                uri: String::from("/v3/accounts/{accountID}/orders/{orderSpecifier}"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return ReplaceOrderRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The Order Specifier
        /// format: Either the Order's OANDA-assigned OrderID or the Order's client-
        /// provided ClientID prefixed by the "@" symbol
        /// - param String
        /// - return ReplaceOrderRequest
        pub fn with_order_specifier(mut self, x: String) -> Self {
            self.path.order_specifier = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return ReplaceOrderRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return ReplaceOrderRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// Client specified RequestID to be sent with request.
        /// - param String
        /// - return ReplaceOrderRequest
        pub fn with_client_request_id(mut self, x: String) -> Self {
            self.header.client_request_id = Some(x);
            self
        }

        /// The base Order specification used when requesting that an Order be
        /// created. Each specific Order-type extends this definition.
        /// - param OrderRequest
        /// - return ReplaceOrderRequest
        pub fn with_order(mut self, x: OrderRequest) -> Self {
            self.body.order = Some(x);
            self
        }

        pub async fn remote(self, client: &Client) -> Result<ReplaceOrderResponse, Box<dyn Error>> {
            let uri = self.uri
                .clone()
                .replace("{accountID}", &self.path.account_id.unwrap())
                .replace("{orderSpecifier}", &self.path.order_specifier.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .put(&url)
                .query(&self.query)
                .json::<RequestBody>(&self.body)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<ReplaceOrderResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type ReplaceOrderResponse = ReplaceOrderResponse200Body;

    /// The Order was successfully cancelled and replaced
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReplaceOrderResponse200Header {
        /// A link to the replacing Order
        #[serde(rename = "Location", skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The Order was successfully cancelled and replaced
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReplaceOrderResponse200Body {
        /// An OrderCancelTransaction represents the cancellation of an Order in
        /// the client's Account.
        #[serde(rename = "orderCancelTransaction", skip_serializing_if = "Option::is_none")]
        pub order_cancel_transaction: Option<OrderCancelTransaction>,
        /// The base Transaction specification. Specifies properties that are
        /// common between all Transaction.
        #[serde(rename = "orderCreateTransaction", skip_serializing_if = "Option::is_none")]
        pub order_create_transaction: Option<Transaction>,
        /// An OrderFillTransaction represents the filling of an Order in the
        /// client's Account.
        #[serde(rename = "orderFillTransaction", skip_serializing_if = "Option::is_none")]
        pub order_fill_transaction: Option<OrderFillTransaction>,
        /// The base Transaction specification. Specifies properties that are
        /// common between all Transaction.
        #[serde(rename = "orderReissueTransaction", skip_serializing_if = "Option::is_none")]
        pub order_reissue_transaction: Option<Transaction>,
        /// The base Transaction specification. Specifies properties that are
        /// common between all Transaction.
        #[serde(rename = "orderReissueRejectTransaction", skip_serializing_if = "Option::is_none")]
        pub order_reissue_reject_transaction: Option<Transaction>,
        /// An OrderCancelTransaction represents the cancellation of an Order in
        /// the client's Account.
        #[serde(
            rename = "replacingOrderCancelTransaction",
            skip_serializing_if = "Option::is_none"
        )]
        pub replacing_order_cancel_transaction: Option<OrderCancelTransaction>,
        /// The IDs of all Transactions that were created while satisfying the
        /// request.
        #[serde(rename = "relatedTransactionIDs", skip_serializing_if = "Option::is_none")]
        pub related_transaction_i_ds: Option<Vec<String>>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod cancel_order {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
        pub accept_datetime_format: Option<String>,

        #[serde(rename = "ClientRequestID", skip_serializing_if = "Option::is_none")]
        pub client_request_id: Option<String>,
    }
    impl RequestHead {
        fn new() -> RequestHead {
            RequestHead {
                authorization: None,
                accept_datetime_format: None,
                client_request_id: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestPath {
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        #[serde(rename = "orderSpecifier", skip_serializing_if = "Option::is_none")]
        pub order_specifier: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath {
                account_id: None,
                order_specifier: None,
            }
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
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Cancel Order
    /// Cancel a pending Order in an Account

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CancelOrderRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl CancelOrderRequest {
        pub fn new() -> CancelOrderRequest {
            CancelOrderRequest {
                uri: String::from("/v3/accounts/{accountID}/orders/{orderSpecifier}/cancel"),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return CancelOrderRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The Order Specifier
        /// format: Either the Order's OANDA-assigned OrderID or the Order's client-
        /// provided ClientID prefixed by the "@" symbol
        /// - param String
        /// - return CancelOrderRequest
        pub fn with_order_specifier(mut self, x: String) -> Self {
            self.path.order_specifier = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return CancelOrderRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return CancelOrderRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// Client specified RequestID to be sent with request.
        /// - param String
        /// - return CancelOrderRequest
        pub fn with_client_request_id(mut self, x: String) -> Self {
            self.header.client_request_id = Some(x);
            self
        }

        pub async fn remote(self, client: &Client) -> Result<CancelOrderResponse, Box<dyn Error>> {
            let uri = self.uri
                .clone()
                .replace("{accountID}", &self.path.account_id.unwrap())
                .replace("{orderSpecifier}", &self.path.order_specifier.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .put(&url)
                .query(&self.query)
                .json::<RequestBody>(&self.body)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<CancelOrderResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type CancelOrderResponse = CancelOrderResponse200Body;

    /// The Order was cancelled as specified
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CancelOrderResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The Order was cancelled as specified
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CancelOrderResponse200Body {
        /// An OrderCancelTransaction represents the cancellation of an Order in
        /// the client's Account.
        #[serde(rename = "orderCancelTransaction", skip_serializing_if = "Option::is_none")]
        pub order_cancel_transaction: Option<OrderCancelTransaction>,
        /// The IDs of all Transactions that were created while satisfying the
        /// request.
        #[serde(rename = "relatedTransactionIDs", skip_serializing_if = "Option::is_none")]
        pub related_transaction_i_ds: Option<Vec<String>>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
    }
}

pub mod set_order_client_extensions {
    #[allow(unused_imports)]
    use chrono::prelude::*;
    #[allow(unused_imports)]
    use fxoanda_definitions::*;
    use std::error::Error;
    use crate::Client;

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestHead {
        #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
        pub authorization: Option<String>,

        #[serde(rename = "AcceptDatetimeFormat", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
        pub account_id: Option<String>,

        #[serde(rename = "orderSpecifier", skip_serializing_if = "Option::is_none")]
        pub order_specifier: Option<String>,
    }
    impl RequestPath {
        fn new() -> RequestPath {
            RequestPath {
                account_id: None,
                order_specifier: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestBody {
        #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
        pub client_extensions: Option<ClientExtensions>,

        #[serde(rename = "tradeClientExtensions", skip_serializing_if = "Option::is_none")]
        pub trade_client_extensions: Option<ClientExtensions>,
    }
    impl RequestBody {
        fn new() -> RequestBody {
            RequestBody {
                client_extensions: None,
                trade_client_extensions: None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestQuery {}
    impl RequestQuery {
        fn new() -> RequestQuery {
            RequestQuery {}
        }
    }

    /// Set Order Extensions
    /// Update the Client Extensions for an Order in an Account. Do not set,
    /// modify, or delete clientExtensions if your account is associated with
    /// MT4.

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetOrderClientExtensionsRequest {
        #[serde(skip_serializing)]
        uri: String,
        header: RequestHead,
        body: RequestBody,
        path: RequestPath,
        query: RequestQuery,
    }

    impl SetOrderClientExtensionsRequest {
        pub fn new() -> SetOrderClientExtensionsRequest {
            SetOrderClientExtensionsRequest {
                uri: String::from(
                    "/v3/accounts/{accountID}/orders/{orderSpecifier}/clientExtensions"
                ),
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

        /// Account Identifier
        /// format: "-"-delimited string with format
        /// "{siteID}-{divisionID}-{userID}-{accountNumber}"
        /// - param String
        /// - return SetOrderClientExtensionsRequest
        pub fn with_account_id(mut self, x: String) -> Self {
            self.path.account_id = Some(x);
            self
        }

        /// The Order Specifier
        /// format: Either the Order's OANDA-assigned OrderID or the Order's client-
        /// provided ClientID prefixed by the "@" symbol
        /// - param String
        /// - return SetOrderClientExtensionsRequest
        pub fn with_order_specifier(mut self, x: String) -> Self {
            self.path.order_specifier = Some(x);
            self
        }

        /// The authorization bearer token previously obtained by the client
        /// format: The string 'Bearer ' followed by the token.
        /// - param String
        /// - return SetOrderClientExtensionsRequest
        pub fn with_authorization(mut self, x: String) -> Self {
            self.header.authorization = Some(x);
            self
        }

        /// Format of DateTime fields in the request and response.
        /// - param String
        /// - return SetOrderClientExtensionsRequest
        pub fn with_accept_datetime_format(mut self, x: String) -> Self {
            self.header.accept_datetime_format = Some(x);
            self
        }

        /// A ClientExtensions object allows a client to attach a clientID, tag
        /// and comment to Orders and Trades in their Account.  Do not set,
        /// modify, or delete this field if your account is associated with MT4.
        /// - param ClientExtensions
        /// - return SetOrderClientExtensionsRequest
        pub fn with_client_extensions(mut self, x: ClientExtensions) -> Self {
            self.body.client_extensions = Some(x);
            self
        }

        /// A ClientExtensions object allows a client to attach a clientID, tag
        /// and comment to Orders and Trades in their Account.  Do not set,
        /// modify, or delete this field if your account is associated with MT4.
        /// - param ClientExtensions
        /// - return SetOrderClientExtensionsRequest
        pub fn with_trade_client_extensions(mut self, x: ClientExtensions) -> Self {
            self.body.trade_client_extensions = Some(x);
            self
        }

        pub async fn remote(
            self,
            client: &Client
        ) -> Result<SetOrderClientExtensionsResponse, Box<dyn Error>> {
            let uri = self.uri
                .clone()
                .replace("{accountID}", &self.path.account_id.unwrap())
                .replace("{orderSpecifier}", &self.path.order_specifier.unwrap());
            let url = format!("https://{host}{uri}", host = client.host, uri = uri);
            let res = client.reqwest
                .put(&url)
                .query(&self.query)
                .json::<RequestBody>(&self.body)
                .bearer_auth(&client.authentication)
                .send().await;
            match res {
                Err(e) => Err(Box::new(e)),
                Ok(response) =>
                    match response.json::<SetOrderClientExtensionsResponse>().await {
                        Err(e) => Err(Box::new(e)),
                        Ok(j) => Ok(j),
                    }
            }
        }
    }

    pub type SetOrderClientExtensionsResponse = SetOrderClientExtensionsResponse200Body;

    /// The Order's Client Extensions were successfully modified
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetOrderClientExtensionsResponse200Header {
        /// The unique identifier generated for the request
        #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
        pub request_id: Option<String>,
    }

    /// The Order's Client Extensions were successfully modified
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetOrderClientExtensionsResponse200Body {
        /// A OrderClientExtensionsModifyTransaction represents the modification
        /// of an Order's Client Extensions.
        #[serde(
            rename = "orderClientExtensionsModifyTransaction",
            skip_serializing_if = "Option::is_none"
        )]
        pub order_client_extensions_modify_transaction: Option<OrderClientExtensionsModifyTransaction>,
        /// The ID of the most recent Transaction created for the Account
        /// format: String representation of the numerical OANDA-assigned TransactionID
        #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
        pub last_transaction_id: Option<String>,
        /// The IDs of all Transactions that were created while satisfying the
        /// request.
        #[serde(rename = "relatedTransactionIDs", skip_serializing_if = "Option::is_none")]
        pub related_transaction_i_ds: Option<Vec<String>>,
    }
}
pub use cancel_order::*;
pub use close_position::*;
pub use close_trade::*;
pub use configure_account::*;
pub use create_limit_order::*;
pub use create_market_order::*;
pub use create_stop_order::*;
pub use get_account::*;
pub use get_account_changes::*;
pub use get_account_instrument_candles::*;
pub use get_account_instruments::*;
pub use get_account_summary::*;
pub use get_order::*;
pub use get_position::*;
pub use get_prices::*;
pub use get_trade::*;
pub use get_transaction::*;
pub use get_transaction_range::*;
pub use get_transactions_since_id::*;
pub use list_accounts::*;
pub use list_open_positions::*;
pub use list_open_trades::*;
pub use list_orders::*;
pub use list_pending_orders::*;
pub use list_positions::*;
pub use list_trades::*;
pub use list_transactions::*;
pub use replace_order::*;
pub use set_order_client_extensions::*;
pub use set_trade_client_extensions::*;
pub use set_trade_dependent_orders::*;
pub use stream_pricing::*;
pub use stream_transactions::*;
