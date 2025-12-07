use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::{collections::HashMap};
use chrono;
use csv;
use std::error::Error;

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

#[derive(Debug)]
struct Candle{
    date: chrono::NaiveDate ,
    price: f64,
    opening_price: f64,
    maximum_price: f64,
    minimum_price: f64,
    exchange_voplume: f64,//unwraped to zero
    price_exchange: f64,//unwraped to zero
}
fn main() { 
    let account_balance = AccountBalancePoint{stocks_holding:0, cash_ammount:100000.0};
    let _aflt = parse_file("D:/2_projects/11_tradingSIm/Raw data/Прошлые данные - AFLT.csv");
    println!("test {:?}", _aflt);
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

fn test_all_scenarios(_stocks_histoty: &Market , account_balance: &AccountBalancePoint){
    for scenario in Scenario::iter(){
        match scenario {
            Scenario::BuySellLoop => {println!("{} {}", account_balance.cash_ammount, account_balance.stocks_holding)},
            Scenario::StartBuyEndSell => {println!("{} {}", account_balance.cash_ammount, account_balance.stocks_holding)},
        }
    }
}



//Parsing
fn numbers_to_f64(s: &str) -> Result<f64, String> {
    let replaced = s.replace(',', ".");
    replaced.parse::<f64>()
        .map_err(|_| format!("Parsing number failed: '{}'", replaced))
}

fn parse_numbers(s: &str) -> Result<f64, String>{
    let s =s.trim_end_matches("%");
    let s: &str=  &s.replace(',', ".");

    if let Some(num) = s.strip_suffix("M") {
        Ok( numbers_to_f64(num)? *1_000_000.0)
    } 
    else if let Some(num) = s.strip_suffix("K") {
        Ok( numbers_to_f64(num)? *1_000.0)
    }
    else {
        numbers_to_f64(&s)
    }
}


fn parse_file(path: &str) -> Result< Vec<Candle>, Box<dyn Error> >{
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    let mut result = Vec::new();

    for recodr in reader.records(){
        let row = recodr?;
        
        let candle = Candle{
            date: chrono::NaiveDate::parse_from_str(&row[0], "%d.%m.%Y")?,
            price: parse_numbers(&row[1])?,
            opening_price: parse_numbers(&row[2])?,
            maximum_price: parse_numbers(&row[3])?,
            minimum_price: parse_numbers(&row[4])?,
            exchange_voplume: parse_numbers(&row[5]).unwrap_or_else(|e| {
                println!("Volume parse error: {}", e);
                0.0
            }),
            price_exchange: parse_numbers(&row[6]).unwrap_or_else(|e| {
                println!("Volume parse error: {}", e);
                0.0
            }
            ),
        };
    result.push(candle);
    }

    Ok(result)
}