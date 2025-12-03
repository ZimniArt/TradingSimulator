
#[derive(Debug)]
struct StockInfo{
    price : f64,
}

fn main() {
    let mut  period: Vec<StockInfo> = Vec::new() ;
    period.push(StockInfo { price: 58.7 });
    period.push(StockInfo { price: 59.3 });
    period.push(StockInfo { price: 61.1 });

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