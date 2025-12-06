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
    raw_data: String,
}

struct DataPoint{
    price : f64,
}

struct AccountBalancePoint{
    stocks_holding: u32,
    cash_ammount:f64,
}
struct Strategy {
    
}
fn main() { 
    let raw_data = RawData{raw_data : String::from("data")};
    let market_timeline: Vec<DataPoint> = parse_data(raw_data);

    let strategy : Strategy = Strategy {};
    let account_starting_point = AccountBalancePoint{stocks_holding:0, cash_ammount:100000.0}
    let account_balance_history : Vec<AccountBalancePoint> = simulate(strategy, market_timeline, account_starting_point);
    
}

fn parse_data(raw_data: RawData) ->Vec<DataPoint>{
    let data_point = DataPoint{ price : 5.3 };
    let mut data_array: Vec<DataPoint> ;
    data_array.push(data_point);
    return data_array
}
fn simulate(strategy: Strategy, market_timeline : Vec<DataPoint>, starting_account: AccountBalancePoint) -> Vec<AccountBalancePoint>{
    let mut account_balance_history : Vec<AccountBalancePoint>;
    account_balance_history.push(starting_account);
    account_balance_history
}

