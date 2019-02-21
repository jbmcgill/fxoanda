use std::error::Error;

#[derive(Debug)]
pub struct Client {
  pub reqwest: reqwest::Client,
  pub host: String,
  pub authentication: String,
} 

macro_rules! client_requests { 
  ( $( $func:ident ( $request:ident ) -> $response:ident ),* ) => {
      $(
         use $request;
         use $response;
         impl Client {
           pub fn $func( &self, x: $request ) -> Result<$response, Box<Error>> {
             x.remote(&self)
           }
         }
       )*
    };
}

client_requests!( candles(GetInstrumentCandlesRequest) -> GetInstrumentCandlesResponse,
                  orderbook(GetOrderBookRequest) -> GetOrderBookResponse,
                  positionbook(GetPositionBookRequest) -> GetPositionBookResponse,
                  pricing(GetPricesRequest) -> GetPricesResponse,
                  accounts(ListAccountsRequest) -> ListAccountsResponse,
                  account_summary(GetAccountSummaryRequest) -> GetAccountSummaryResponse);
