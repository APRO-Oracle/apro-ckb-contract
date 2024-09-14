extern crate alloc;

use alloc::vec::Vec;
use ckb_std::ckb_constants::Source;
// use ckb_std::ckb_types::packed::{BytesBuilder, WitnessArgsBuilder};

use ckb_std::debug;
use ckb_std::error::SysError;

use ckb_std::high_level::load_cell_data;

// use hex::decode;
use molecule::prelude::Reader;
// use stablepp_core::types::cell_data::oracle_data::OracleData;
// use stablepp_core::types::cell_data::oracle_data::OracleDataReader;
// use stablepp_core::types::errors::Error;
use crate::oracle_data::oracle_data::OracleData;
use crate::oracle_data::oracle_data::OracleDataReader;
use crate::common::error::Error;

pub fn run() -> Result<(), Error> {
    let mut input_cell_count: Vec<OracleData> = Vec::new();
    let mut output_cell_count: Vec<OracleData> = Vec::new();
    let mut index = 0;
    loop {
        let data = load_cell_data(index, Source::GroupInput);
        match data {
            Ok(data) => {
                if data.starts_with(b"OracleCell") {
                    let oracle_data = OracleDataReader::from_compatible_slice(&data["OracleCell".len()..])
                        .map_err(|_| Error::InvalidOracleCellData)?
                        .to_entity();
                    input_cell_count.push(oracle_data);
                }
                // debug!("OracleCell found input" );
                debug!("OracleCell found in put{}", input_cell_count.len());
            }
            Err(SysError::IndexOutOfBound) => {
                break;
            }
            Err(_) => {
                return Err(Error::IncorrectCellAmount);
            }
        }
        index += 1;
    }
    index = 0;
    loop {
        let data = load_cell_data(index, Source::GroupOutput);
        match data {
            Ok(data) => {
                if data.starts_with(b"OracleCell") {
                    let oracle_data = OracleDataReader::from_compatible_slice(&data["OracleCell".len()..])
                        .map_err(|_| Error::InvalidOracleCellData)?
                        .to_entity();
                    output_cell_count.push(oracle_data);
                }
                debug!("OracleCell found out put{}", output_cell_count.len());
            }
            Err(SysError::IndexOutOfBound) => {
                break;
            }
            Err(_) => {
                return Err(Error::IncorrectCellAmount);
            }
        }
        index += 1;
    }
    debug!("input_cell_count.len{}", input_cell_count.len());
    debug!("output_cell_count.len{}", output_cell_count.len());
    if input_cell_count.len() > 1 {
        return Err(Error::IncorrectCellInputAmount);
    }
    if output_cell_count.len() != 1 {
        return Err(Error::IncorrectCellOutputAmount);
    }
    let price_data_vec = output_cell_count[0].price_data();
    if price_data_vec.is_empty() {
        return Err(Error::WrongOracleData);
    }

    Ok(())
}
