use prettytable::{
    format::{FormatBuilder, LinePosition, LineSeparator},
    row, Table,
};
use test_struct::IntermediateAggregationResults;

use crate::test_struct::get_test_struct;

use anyhow::Result;
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

fn main() {
    let test_struct = get_test_struct();

    let mut table = get_markdown_table();
    table.set_titles(row!["Format", "Result",]);

    table.add_row(row!["Json", format!("{:?}", test_json(&test_struct))]);
    table.add_row(row![
        "Postcard ",
        format!("{:?}", test_postcard(&test_struct))
    ]);
    table.add_row(row!["Ron", format!("{:?}", test_ron(&test_struct))]);
    table.add_row(row![
        "MessagePack ",
        format!("{:?}", test_rmp(&test_struct))
    ]);
    table.add_row(row!["Bincode", format!("{:?}", test_bincode(&test_struct))]);
    table.add_row(row![
        "Ciborium ",
        format!("{:?}", test_ciborium(&test_struct))
    ]);
    table.add_row(row!["Bson", format!("{:?}", test_bson(&test_struct))]);
    table.printstd();
}

fn test_json(test_struct: &IntermediateAggregationResults) -> Result<()> {
    let output = serde_json::to_string(test_struct)?;
    let deser: IntermediateAggregationResults = serde_json::from_str(&output)?;
    cmp_res(test_struct, &deser)?;
    Ok(())
}

fn test_postcard(test_struct: &IntermediateAggregationResults) -> Result<()> {
    let output: Vec<u8> = postcard::to_allocvec(test_struct)?;
    let _deser: IntermediateAggregationResults = postcard::from_bytes(&output)?;
    cmp_res(test_struct, &_deser)?;
    Ok(())
}

fn test_ron(test_struct: &IntermediateAggregationResults) -> Result<()> {
    let output = ron::to_string(test_struct)?;
    let _deser: IntermediateAggregationResults = ron::from_str(&output)?;
    cmp_res(test_struct, &_deser)?;
    Ok(())
}

fn test_rmp(test_struct: &IntermediateAggregationResults) -> Result<()> {
    let output = rmp_serde::to_vec(test_struct)?;
    let _deser: IntermediateAggregationResults = rmp_serde::from_slice(&output)?;
    cmp_res(test_struct, &_deser)?;
    Ok(())
}

fn test_bincode(test_struct: &IntermediateAggregationResults) -> Result<()> {
    let output = bincode::serialize(test_struct)?;
    let _deser: IntermediateAggregationResults = bincode::deserialize(&output)?;
    cmp_res(test_struct, &_deser)?;
    Ok(())
}

fn test_ciborium(test_struct: &IntermediateAggregationResults) -> Result<()> {
    let mut output = Vec::new();
    ciborium::ser::into_writer(test_struct, &mut output)?;
    let _deser: IntermediateAggregationResults = ciborium::de::from_reader(&mut output.as_slice())?;
    cmp_res(test_struct, &_deser)?;
    Ok(())
}

fn test_bson(test_struct: &IntermediateAggregationResults) -> Result<()> {
    let output = bson::to_bson(test_struct)?;
    let _deser: IntermediateAggregationResults = bson::from_bson(output)?;
    cmp_res(test_struct, &_deser)?;
    Ok(())
}

fn cmp_res(
    orig: &IntermediateAggregationResults,
    deser: &IntermediateAggregationResults,
) -> anyhow::Result<()> {
    if orig != deser {
        anyhow::bail!("Equality Missmatch");
    }
    Ok(())
}
