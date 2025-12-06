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
#[derive(Debug)]
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
    let _data: Vec<DataPoint> = parse_data(_raw_data);
    let account_balance = AccountBalancePoint{stocks_holding:0, cash_ammount:100000.0};
    println!(
        "current stocks holding is {} and amount of cash is {} rubles",
        &account_balance.stocks_holding, 
        &account_balance.cash_ammount
    );   

    
    test_all_strategies(_data, &account_balance);
    let _raw_data = RawData{_raw_data : String::from("data")};
    let _example_data = parse_data(_raw_data);
    test_all_strategies(_example_data,&account_balance);


    // let _strategy   = StrategyStartBuyEndSell {};
    // simulate(&_strategy, &_market_timeline, &_account_starting_point);

    // let _strategy   = StategyLoopBuySell {};
    // simulate(&_strategy, &_market_timeline, &_account_starting_point);
    
}

fn test_all_strategies(_data:  Vec<DataPoint>, account_balance: &AccountBalancePoint){
    let _strategy   = StrategyStartBuyEndSell {};
    simulate(&_strategy, &_data, &account_balance);

    let _strategy   = StategyLoopBuySell {};
    simulate(&_strategy, &_data, &account_balance);
}

fn parse_data(_raw_data: RawData) ->Vec<DataPoint>{

    let data_array: Vec<DataPoint> = vec![DataPoint{ price : 5.3 }, DataPoint{ price : 5.6 },DataPoint{ price : 5.9 }];
    return data_array
}

fn simulate <T: Strategy + std::fmt::Debug>(_strategy: &T, _market_timeline : &Vec<DataPoint>, starting_account: &AccountBalancePoint){
    println!("\nmarket point is {:?}", _market_timeline[0].price);
    println!("market point is {:?}", _market_timeline[1].price);
    println!("market point is {:?}", _market_timeline[2].price);

    let  _account_balance_history : Vec<AccountBalancePoint>= vec![starting_account.clone()];

    println!("{:?} resulted in {:?}",_strategy, _account_balance_history)
}

