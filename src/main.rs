use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

fn main() { 
    let account_balance = AccountBalancePoint{stocks_holding:0, cash_ammount:100000.0};

    for company in Company::iter(){
        parse_data(&company);
        test_all_scenarios(&company, &account_balance);
    }
    
}
fn parse_data(_company: &Company){

}
fn test_all_scenarios(_company: &Company , account_balance: &AccountBalancePoint){
    for scenario in Scenario::iter(){
        match scenario {
            Scenario::BuySellLoop => {println!("{} {}", account_balance.cash_ammount, account_balance.stocks_holding)},
            Scenario::StartBuyEndSell => {println!("{} {}", account_balance.cash_ammount, account_balance.stocks_holding)},
        }
    }
}

