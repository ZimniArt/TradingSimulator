use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use core::f64;
use std::{collections::HashMap};
use chrono;
use csv;
use std::error::Error;
use plotters::prelude::*;

#[derive(Debug)]
#[derive(Clone)]
struct AccountBalancePoint{
    stocks_holding: u32,
    cash_ammount:f64,
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
    let _aflt = parse_file("D:/2_projects/11_tradingSIm/Raw data/Прошлые данные - AFLT.csv").unwrap();
    let _afit_prices: Vec<f64> = _aflt.iter().map(|c|c.price).collect();
    let _aflt_graph_data = _afit_prices.clone();
    draw_chart(_aflt_graph_data.len() as f64,_aflt_graph_data, "D:/2_projects/11_tradingSIm/Graphics/test.svg");
    
    let _balance_history =play_scenario(_afit_prices, &account_balance);
    draw_chart(_balance_history.len() as f64,_balance_history, "D:/2_projects/11_tradingSIm/Graphics/balance_history.svg");
    
  
}

fn play_scenario(_stock_prices: Vec<f64>, account_balance: &AccountBalancePoint) ->Vec<f64> {
    let mut _account_history: Vec<f64> = vec![account_balance.clone().cash_ammount];
    let mut _account_balance = account_balance.clone();
    let commissioned: f64 = 0.06;
    for day in _stock_prices{
        if _account_balance.cash_ammount>day{
            let _new_stocks =((_account_balance.cash_ammount*(1.0-&commissioned))/day).floor() as u32;
            _account_balance.stocks_holding  += _new_stocks;
            _account_balance.cash_ammount -=(day*(1.0+&commissioned))*(_new_stocks as f64);
        }
        else{
            let _stocks_to_sell= _account_balance.stocks_holding;
            let _new_cash = day*(1.0+&commissioned) *(_stocks_to_sell as f64);
            _account_balance.cash_ammount +=_new_cash;
            _account_balance.stocks_holding  = 0;
        }
        _account_history.push(_account_balance.cash_ammount);
    }
    _account_history
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
// Draw char
fn draw_chart(_horizontal: f64, _vertical: Vec<f64>, output: &str) -> Result<(), Box<dyn Error>>  {
    let root = SVGBackend::new(output, (1200,600)).into_drawing_area();
    root.fill(&WHITE)?;

     
    
    let (xmin,xmax)= (0.0,_horizontal);
    let ymin = 0.0;
    let ymax = _vertical.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("chart example",("sans-serfi", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(xmin..xmax, ymin..ymax)?;

    chart.configure_mesh().draw();

    let data: Vec<(f64,f64)> = _vertical.iter().enumerate().map(|(i,v)| (i as f64, *v)).collect();
    
    chart
        .draw_series(LineSeries::new(data, &RED))?
        .label("some label")
        .legend(|(x,y)| PathElement::new([(x,y), (x+20,y)], &RED));
    chart.configure_series_labels().draw()?;
    Ok(())
    

}