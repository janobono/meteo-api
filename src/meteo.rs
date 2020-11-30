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
    Datetime,
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
    UV,
}

impl FromStr for Column {
    type Err = ();

    fn from_str(input: &str) -> Result<Column, Self::Err> {
        match input {
            "datetime" => Ok(Column::Datetime),
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
            "UV" => Ok(Column::UV),
            _ => Err(()),
        }
    }
}

pub fn to_sql(query_input: &QueryInput) -> (Vec<Column>, String) {
    println!("{:?}", query_input);

    let columns = query_input.columns.split(',').map(|v| { Column::from_str(v).expect("Unsupported column name!") }).collect();
    let mut sql = String::from("SELECT datetime,");
    sql = sql.add(query_input.columns.as_str());
    sql = sql.add(format!(" FROM archive WHERE datetime >= {} AND datetime <= {}", query_input.timestamp_from, query_input.timestamp_to).as_str());

    println!("columns = {:?}", columns);
    println!("sql = {}", sql);
    (columns, sql)
}

pub fn to_json(query_result: &mut mysql::QueryResult<mysql::Text>) -> String {
    let mut result = String::from("[");
    let mut count = 0;

    for query_row in query_result {
        let mut row_string = String::from("{");

        let row = query_row.unwrap();
        let row_columns = row.columns();
        let row_values = row.unwrap();

        let mut index = 0;
        for row_column in row_columns.iter() {
            row_string.push('"');
            row_string = row_string.add(row_column.name_str().as_ref());
            row_string.push('"');
            row_string.push(':');
            row_string = row_string.add(value_to_json(&row_values[index]).as_str());
            row_string.push(',');
            index += 1;
        }
        row_string.pop();
        row_string.push('}');
        println!("row {} = {}", count, row_string);

        result = result.add(row_string.as_str());
        result.push(',');
        count += 1;
    }

    result.pop();
    result.push(']');

    println!("rows count {}", count);
    result
}

fn value_to_json(value: &mysql::Value) -> String {
    match value {
        mysql::Value::NULL => String::from("null"),
        mysql::Value::Int(val) => format!("{}", val),
        mysql::Value::Bytes(_) => {
            let value_str = String::from(value.as_sql(true).as_str()).replace("'", "");
            if value_str.contains(".") {
                let f = value_str.parse::<f32>().unwrap();
                format!("{:.2}", f)
            } else {
                value_str
            }
        }
        _ => String::from("err"),
    }
}
