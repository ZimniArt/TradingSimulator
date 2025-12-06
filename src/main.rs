use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::collections::HashMap;
use chrono;

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
#[derive(EnumIter, Debug,Eq, Hash, PartialEq)]
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
    fn add_company(&mut self, _company :Company, _hisory: StocksHistory){
        self.companies_history.insert(_company,_hisory);
    }
}

struct Candle{
    date: chrono::NaiveDate ,
    price: f64,
    opening_price: f64,
    maximum_price: f64,
    minimum_price: f64,
    exchange_voplume: f64,
    price_exchange: f64,
}
fn main() { 
    let account_balance = AccountBalancePoint{stocks_holding:0, cash_ammount:100000.0};

    let mut _stocks_histoty= parse_data();
    test_all_scenarios(&_stocks_histoty, &account_balance);
    
}
fn parse_data() -> Market {
    let mut _stocks_histoty: Market = Market::new(); 
    for _copmany in Company::iter(){
        let temp_history = StocksHistory::new();
        _stocks_histoty.add_company(_copmany, temp_history);
    }
    _stocks_histoty
}

fn load_data(_copmany:Company) {}
fn clean_headers(){}

fn test_all_scenarios(_stocks_histoty: &Market , account_balance: &AccountBalancePoint){
    for scenario in Scenario::iter(){
        match scenario {
            Scenario::BuySellLoop => {println!("{} {}", account_balance.cash_ammount, account_balance.stocks_holding)},
            Scenario::StartBuyEndSell => {println!("{} {}", account_balance.cash_ammount, account_balance.stocks_holding)},
        }
    }
}

