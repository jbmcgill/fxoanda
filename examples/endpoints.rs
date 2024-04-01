extern crate fxoanda;
use fxoanda::*;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("OANDA_KEY").expect("expected OANDA_KEY environment variable to be set");
    let api_host = env
        ::var("OANDA_HOST")
        .expect("expected OANDA_HOST environment variable to be set");

    // only run example program against demo account!!
    assert_eq!(api_host, "api-fxpractice.oanda.com");

    let client = fxoanda::Client {
        host: String::from(api_host),
        reqwest: reqwest::Client::new(),
        authentication: String::from(api_key),
    };

    match
        fxoanda::GetInstrumentCandlesRequest
            ::new()
            .with_instrument("EUR_USD".to_string())
            .with_granularity(CandlestickGranularity::H4)
            .remote(&client).await
    {
        Ok(x) => println!("OK: {:#?}", x),
        Err(e) => eprintln!("ERR: {:#?}", e),
    }

    match
        fxoanda::GetPositionBookRequest
            ::new()
            .with_instrument("EUR_USD".to_string())
            .remote(&client).await
    {
        Ok(x) => println!("OK: {:#?}", x),
        Err(e) => println!("ERR: {:#?}", e),
    }

    match
        fxoanda::GetOrderBookRequest
            ::new()
            .with_instrument("EUR_USD".to_string())
            .remote(&client).await
    {
        Ok(x) => println!("OK: {:#?}", x),
        Err(e) => println!("ERR: {:#?}", e),
    }

    match fxoanda::ListAccountsRequest::new().remote(&client).await {
        Ok(x) => println!("OK: {:#?}", x),
        Err(e) => println!("ERR: {:#?}", e),
    }

    let response = fxoanda::ListAccountsRequest::new().remote(&client).await.unwrap();
    let accounts = response.accounts.expect("Did not find 'accounts' field in response");
    let account = accounts.get(0).expect("Did not find an 'account' in the 'accounts' field");
    let account_id = account.id
        .as_ref()
        .expect("Did not find an 'id' field in the 'account'")
        .to_string();

    match
        fxoanda::GetAccountSummaryRequest
            ::new()
            .with_account_id(account_id.to_owned())
            .remote(&client).await
    {
        Ok(x) => println!("OK: {:#?}", x),
        Err(e) => println!("ERR: {:#?}", e),
    }

    match
        fxoanda::GetAccountRequest
            ::new()
            .with_account_id(account_id.to_owned())
            .remote(&client).await
    {
        Ok(x) => println!("OK: {:#?}", x),
        Err(e) => println!("ERR: {:#?}", e),
    }

    match
        fxoanda::GetAccountInstrumentsRequest
            ::new()
            .with_account_id(account_id.to_owned())
            .remote(&client).await
    {
        Ok(x) => println!("OK: {:#?}", x),
        Err(e) => println!("ERR: {:#?}", e),
    }

    match
        fxoanda::ListOrdersRequest
            ::new()
            .with_account_id(account_id.to_owned())
            .remote(&client).await
    {
        Ok(x) => println!("OK: {:#?}", x),
        Err(e) => println!("ERR: {:#?}", e),
    }

    match
        fxoanda::ListPendingOrdersRequest
            ::new()
            .with_account_id(account_id.to_owned())
            .remote(&client).await
    {
        Ok(x) => println!("OK: {:#?}", x),
        Err(e) => println!("ERR: {:#?}", e),
    }

    match
        fxoanda::ListTradesRequest
            ::new()
            .with_account_id(account_id.to_owned())
            .remote(&client).await
    {
        Ok(x) => println!("OK: {:#?}", x),
        Err(e) => println!("ERR: {:#?}", e),
    }

    match
        fxoanda::ListOpenTradesRequest
            ::new()
            .with_account_id(account_id.to_owned())
            .remote(&client).await
    {
        Ok(x) => println!("OK: {:#?}", x),
        Err(e) => println!("ERR: {:#?}", e),
    }

    match
        fxoanda::ListPositionsRequest
            ::new()
            .with_account_id(account_id.to_owned())
            .remote(&client).await
    {
        Ok(x) => println!("OK: {:#?}", x),
        Err(e) => println!("ERR: {:#?}", e),
    }

    match
        fxoanda::ListOpenPositionsRequest
            ::new()
            .with_account_id(account_id.to_owned())
            .remote(&client).await
    {
        Ok(x) => println!("OK: {:#?}", x),
        Err(e) => println!("ERR: {:#?}", e),
    }

    match
        fxoanda::ListTransactionsRequest
            ::new()
            .with_account_id(account_id.to_owned())
            .remote(&client).await
    {
        Ok(x) => println!("OK: {:#?}", x),
        Err(e) => println!("ERR: {:#?}", e),
    }
}
