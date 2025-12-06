use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::collections::HashMap;


#[derive(Debug)]
//#[derive(Clone)]
struct AccountBalancePoint{
    stocks_holding: u32,
    cash_ammount:f64,
}

    
#[derive(EnumIter, Debug)]
enum Scenario{
    StartBuyEndSell,
    BuySellLoop,
}
#[derive(EnumIter, Debug)]
enum Company{
    Yandex,
    Example,
}

struct StockInfo{
    price : i32,
}
type StocksHistory = Vec<StockInfo>;
struct Market{
    companies_history: HashMap<Company, StocksHistory>,
}
impl Market{
    fn new()  -> Self{
        Self { companies_history: HashMap::new() }
    }
}

// struct Candle{
//     date: chrono ,
//     price: f64,
//     opening_price: f64,
//     maximum_price: f64,
//     minimum_price: f64,
//     exchange_voplume: i32,
//     price_exchange: f64,
// }
fn main() { 
    let account_balance = AccountBalancePoint{stocks_holding:0, cash_ammount:100000.0};

    for company in Company::iter(){
        let mut _stocks_histoty= parse_data(&company);
        test_all_scenarios(&_stocks_histoty, &account_balance);
    }
    
}
fn parse_data(_company: &Company) -> Market {
    let mut _stocks_histoty: Market = Market::new();
    _stocks_histoty
}
fn test_all_scenarios(_stocks_histoty: &Market , account_balance: &AccountBalancePoint){
    for scenario in Scenario::iter(){
        match scenario {
            Scenario::BuySellLoop => {println!("{} {}", account_balance.cash_ammount, account_balance.stocks_holding)},
            Scenario::StartBuyEndSell => {println!("{} {}", account_balance.cash_ammount, account_balance.stocks_holding)},
        }
    }
}

