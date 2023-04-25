use std::fmt;

use prettytable::cell;

use formats::{Bincode, Bitcode, Ciborium, Json, Ron};
use prettytable::{
    format::{FormatBuilder, LinePosition, LineSeparator},
    row, Row, Table,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::test_struct::get_test_struct;

use anyhow::Result;
mod formats;
mod test_struct;

fn get_markdown_table() -> Table {
    let mut table = Table::new();

    let minus_pipe_sep: LineSeparator = LineSeparator::new('-', '|', '|', '|');
    let format_markdown = FormatBuilder::new()
        .padding(1, 1)
        .borders('|')
        .separator(LinePosition::Title, minus_pipe_sep)
        .column_separator('|')
        .build();
    table.set_format(format_markdown);

    table
}

trait Deser {
    type Serialized: fmt::Debug;
    fn name() -> String;
    fn serialize<T: Serialize>(t: &T) -> Result<(usize, Self::Serialized)>;
    fn deserialize<T: DeserializeOwned>(s: Self::Serialized) -> Result<T>;
}

fn main() {
    let test_struct = get_test_struct();

    let mut table = get_markdown_table();
    table.set_titles(row![
        "Format",
        "Result",
        "Serialized Size",
        "Serialize Time [ns]",
        "Deserialize Time [ns]"
    ]);
    table.add_row(get_row_for_format::<_, Json>(&test_struct));
    table.add_row(get_row_for_format::<_, Ron>(&test_struct));
    table.add_row(get_row_for_format::<_, Bincode>(&test_struct));
    //table.add_row(get_row_for_format::<_, Bson>(&test_struct));
    table.add_row(get_row_for_format::<_, Bitcode>(&test_struct));
    table.add_row(get_row_for_format::<_, Ciborium>(&test_struct));
    table.printstd();
}

struct FormatResult {
    serialize_time: u128,
    deserialize_time: u128,
    serialized_size: usize,
    result: String,
}

fn get_row_for_format<T: PartialEq + Serialize + DeserializeOwned + std::fmt::Debug, F: Deser>(
    test_struct: &T,
) -> Row {
    let res = test_format::<T, F>(test_struct);

    let mut row = Row::empty();
    row.add_cell(cell!(F::name()));
    row.add_cell(cell!(res.result));
    row.add_cell(cell!(res.serialized_size));
    row.add_cell(cell!(res.serialize_time));
    row.add_cell(cell!(res.deserialize_time));
    row
}

fn test_format<T: PartialEq + Serialize + DeserializeOwned + std::fmt::Debug, F: Deser>(
    test_struct: &T,
) -> FormatResult {
    let start = std::time::Instant::now();
    let output = F::serialize(test_struct);
    if output.is_err() {
        return FormatResult {
            serialize_time: 0,
            deserialize_time: 0,
            serialized_size: 0,
            result: format!("Ser Err: {:?}", output.unwrap_err()),
        };
    }

    let serialize_time = std::time::Instant::now() - start;
    let start = std::time::Instant::now();
    let (serialized_size, output) = output.unwrap();
    let deser: Result<T> = F::deserialize(output);

    if deser.is_err() {
        return FormatResult {
            serialize_time: serialize_time.as_nanos(),
            deserialize_time: 0,
            serialized_size,
            result: format!("Deser Err: {:?}", deser.unwrap_err()),
        };
    }

    let deserialize_time = std::time::Instant::now() - start;
    let cmp = cmp_res(test_struct, &deser.unwrap());
    let result = if cmp.is_err() { "Cmp Mismatch" } else { "Ok" };
    FormatResult {
        serialize_time: serialize_time.as_nanos(),
        deserialize_time: deserialize_time.as_nanos(),
        serialized_size,
        result: result.to_string(),
    }
}

fn cmp_res<T: PartialEq>(orig: &T, deser: &T) -> anyhow::Result<()> {
    if orig != deser {
        anyhow::bail!("Equality Missmatch");
    }
    Ok(())
}
