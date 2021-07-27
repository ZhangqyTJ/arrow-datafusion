// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use ballista::prelude::*;
use datafusion::arrow::util::pretty;
use datafusion::prelude::CsvReadOptions;

/// This example demonstrates executing a simple query against an Arrow data source (CSV) and
/// fetching results, using SQL
#[tokio::main]
async fn main() -> Result<()> {
    let config = BallistaConfig::builder()
        .set("ballista.shuffle.partitions", "4")
        .build()?;
    let ctx = BallistaContext::remote("localhost", 50050, &config);

    let testdata = datafusion::arrow::util::test_util::arrow_test_data();

    // register csv file with the execution context
    ctx.register_csv(
        "aggregate_test_100",
        &format!("{}/csv/aggregate_test_100.csv", testdata),
        CsvReadOptions::new(),
    )?;

    // execute the query
    let df = ctx.sql(
        "SELECT c1, MIN(c12), MAX(c12) \
        FROM aggregate_test_100 \
        WHERE c11 > 0.1 AND c11 < 0.9 \
        GROUP BY c1",
    )?;

    // execute the query - note that calling collect on the DataFrame
    // trait will execute the query with DataFusion so we have to call
    // collect on the BallistaContext instead and pass it the DataFusion
    // logical plan
    let mut stream = ctx.collect(&df.to_logical_plan()).await?;

    // print the results
    let mut results = vec![];
    while let Some(batch) = stream.next().await {
        let batch = batch?;
        results.push(batch);
    }
    pretty::print_batches(&results)?;

    Ok(())
}