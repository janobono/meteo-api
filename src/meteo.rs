use std::ops::Add;
use std::str::FromStr;

use serde::Deserialize;

pub fn env(key: &str, def: &str) -> String {
    match std::env::var(key) {
        Ok(value) => value,
        Err(_err) => String::from(def)
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryInput {
    #[serde(rename = "timestampFrom")]
    pub timestamp_from: i64,
    #[serde(rename = "timestampTo")]
    pub timestamp_to: i64,
    pub columns: String,
}

#[derive(Debug, PartialEq)]
pub enum Column {
    InTemp,
    OutTemp,
    Barometer,
    InHumidity,
    OutHumidity,
    WindSpeed,
    WindDir,
    WindGust,
    WindGustDir,
    RainRate,
    Rain,
    Dewpoint,
    Windchill,
    Heatindex,
    U,
}

impl FromStr for Column {
    type Err = ();

    fn from_str(input: &str) -> Result<Column, Self::Err> {
        match input {
            "inTemp" => Ok(Column::InTemp),
            "outTemp" => Ok(Column::OutTemp),
            "barometer" => Ok(Column::Barometer),
            "inHumidity" => Ok(Column::InHumidity),
            "outHumidity" => Ok(Column::OutHumidity),
            "windSpeed" => Ok(Column::WindSpeed),
            "windDir" => Ok(Column::WindDir),
            "windGust" => Ok(Column::WindGust),
            "windGustDir" => Ok(Column::WindGustDir),
            "rainRate" => Ok(Column::RainRate),
            "rain" => Ok(Column::Rain),
            "dewpoint" => Ok(Column::Dewpoint),
            "windchill" => Ok(Column::Windchill),
            "heatindex" => Ok(Column::Heatindex),
            "U" => Ok(Column::U),
            _ => Err(()),
        }
    }
}

impl Column {
    pub fn to_string(&self) -> String {
        match self {
            Column::InTemp => String::from("inTemp"),
            Column::OutTemp => String::from("outTemp"),
            Column::Barometer => String::from("barometer"),
            Column::InHumidity => String::from("inHumidity"),
            Column::OutHumidity => String::from("outHumidity"),
            Column::WindSpeed => String::from("windSpeed"),
            Column::WindDir => String::from("windDir"),
            Column::WindGust => String::from("windGust"),
            Column::WindGustDir => String::from("windGustDir"),
            Column::RainRate => String::from("rainRate"),
            Column::Rain => String::from("rain"),
            Column::Dewpoint => String::from("dewpoint"),
            Column::Windchill => String::from("windchill"),
            Column::Heatindex => String::from("heatindex"),
            Column::U => String::from("U"),
        }
    }
}

pub fn to_sql(query_input: &QueryInput) -> (Vec<Column>, String) {
    let columns = query_input.columns.split(',').map(|v| { Column::from_str(v).expect("Unsupported column name!") }).collect();

    let mut sql = String::from("SELECT datetime,");
    sql = sql.add(query_input.columns.as_str());
    sql = sql.add(format!(" FROM archive WHERE datetime >= {} AND datetime <= {}", query_input.timestamp_from, query_input.timestamp_to).as_str());

    (columns, sql)
}

pub fn to_json(columns: &Vec<Column>, rows: &mut mysql::QueryResult<mysql::Text>) -> String {
    let mut result = String::from("[");

    let mut count = 0;
    for row in rows {
        result.push('{');

        for column in columns {
            result.push('"');
            result = result.add(column.to_string().as_str());
            result.push('"');
            result.push(':');
// TODO
            result.push(',');
        }

        result.pop();
        result.push('}');
        result.push(',');
        count += 1;
    }
    println!("rows count {}", count);
    result.pop();
    result.push(']');
    result
}
