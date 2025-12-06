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
struct AccountBalancePoint{
    stocks_holding: u32,
    cash_ammount:f64,
}
struct Strategy {
    
}
fn main() { 
    let _raw_data = RawData{_raw_data : String::from("data")};
    let _market_timeline: Vec<DataPoint> = parse_data(_raw_data);

    let _strategy : Strategy = Strategy {};
    let _account_starting_point = AccountBalancePoint{stocks_holding:0, cash_ammount:100000.0};
    println!("current stocks holding is {} and amount of cash is {} rubles", &_account_starting_point.stocks_holding, &_account_starting_point.cash_ammount);
    let _account_balance_history : Vec<AccountBalancePoint> = simulate(_strategy, _market_timeline, _account_starting_point);

    println!("account is {:?}", _account_balance_history)
}

fn parse_data(_raw_data: RawData) ->Vec<DataPoint>{
    let data_point = DataPoint{ price : 5.3 };
    let data_array: Vec<DataPoint> = vec![data_point];
    return data_array
}
fn simulate(_strategy: Strategy, _market_timeline : Vec<DataPoint>, starting_account: AccountBalancePoint) -> Vec<AccountBalancePoint>{
    println!("market point is {}", _market_timeline[0].price);
    let mut _account_balance_history : Vec<AccountBalancePoint>= vec![starting_account];
    _account_balance_history
}

