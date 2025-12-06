// struct RawDataPoint{
//     date: String,
//     price: f64,
//     opening_price: f64,
//     maximum_price: f64,
//     minimum_price: f64,
//     exchange_voplume: i32,
//     price_exchange: f64,
// }
struct RawData{
    _raw_data: String,
}

struct DataPoint{
    price : f64,
}

#[derive(Debug)]
#[derive(Clone)]
struct AccountBalancePoint{
    stocks_holding: u32,
    cash_ammount:f64,
}

trait Strategy {}
#[derive(Debug)]
struct StategyLoopBuySell{}
impl Strategy for StategyLoopBuySell {}

#[derive(Debug)]
struct StrategyStartBuyEndSell{}
impl Strategy for StrategyStartBuyEndSell{}

// enum Action{
//     Sell {amount: i32},
//     Buy {amount: i32},
//     Hold,
// }

fn main() { 
    let _raw_data = RawData{_raw_data : String::from("data")};
    let _market_timeline: Vec<DataPoint> = parse_data(_raw_data);

    // let _strategy   = StategyLoopBuySell {};
    let _strategy   = StrategyStartBuyEndSell {};
    let _account_starting_point = AccountBalancePoint{stocks_holding:0, cash_ammount:100000.0};
    
    println!("current stocks holding is {} and amount of cash is {} rubles", &_account_starting_point.stocks_holding, &_account_starting_point.cash_ammount);
    
    simulate(&_strategy, &_market_timeline, &_account_starting_point);

    let _strategy   = StategyLoopBuySell {};
    simulate(&_strategy, &_market_timeline, &_account_starting_point);
    


}


fn parse_data(_raw_data: RawData) ->Vec<DataPoint>{
    let data_point = DataPoint{ price : 5.3 };
    let data_array: Vec<DataPoint> = vec![data_point];
    return data_array
}

fn simulate <T: Strategy + std::fmt::Debug>(_strategy: &T, _market_timeline : &Vec<DataPoint>, starting_account: &AccountBalancePoint){
    println!("\nmarket point is {}", _market_timeline[0].price);
    let  _account_balance_history : Vec<AccountBalancePoint>= vec![starting_account.clone()];
    println!("{:?} resulted in {:?}",_strategy, _account_balance_history)
}

