#[derive(Debug)]
enum Stock {
    Yandex,
    Aeroflot
}
#[derive(Debug)]
struct StockInPortfolio{
    name: Stock,
    amount: u32
}
#[derive(Debug)]
struct StockInfo{
    name: Stock,
    price : f64,
}
#[derive(Debug)]
struct Portfolo{
    stocks: Vec<StockInPortfolio>,
}


enum Action {
    Buy,
    Sell,
    Hold
}


fn main() {
    let mut  period: Vec<StockInfo> = Vec::new() ;
    period.push(StockInfo {name: Stock::Yandex, price: 58.7 });
    period.push(StockInfo {name: Stock::Yandex, price: 59.3 });
    period.push(StockInfo {name: Stock::Yandex, price: 61.1 });

    println!("example of data is {:?}", period);



}




fn _behavior_1_buy(){
    //prpbably buy at the start, sell at the end
}

fn _transaction(){
    // probably will have a commission per every operation
}


// import raw data: date, average\midian price of the day

//money in

//siimulation loop
//behavior

//graphic visualisation of data, stock prices\ money
//result. money out