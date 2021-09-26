use chrono::DateTime;
use chrono::Utc;
use env_logger::{Builder, Env};
use log::info;
use prometheus_exporter::prometheus::register_gauge;
use serde::Deserialize;
use std::net::SocketAddr;
use std::env;

/*
example:
{
    "timestamp":"2021-09-25T16:51:10.398Z",
    "score":97,
    "dew_point":9.42,
    "temp":20.68,
    "humid":48.48,
    "abs_humid":8.70,
    "co2":626,
    "co2_est":410,
    "co2_est_baseline":36354,
    "voc":83,
    "voc_baseline":38852,
    "voc_h2_raw":26,
    "voc_ethanol_raw":38,
    "pm25":1,
    "pm10_est":2
}
 */
#[derive(Debug, Deserialize)]
struct AwairLocalAirDataResponse {
    timestamp: DateTime<Utc>,
    score: f64,
    dew_point: f64,
    temp: f64,
    humid: f64,
    abs_humid: f64,
    co2: f64,
    co2_est: f64,
    co2_est_baseline: f64,
    voc: f64,
    voc_baseline: f64,
    voc_h2_raw: f64,
    voc_ethanol_raw: f64,
    pm25: f64,
    pm10_est: f64,
}

fn main() {
    // Setup logger with default level info so we can see the messages from
    // prometheus_exporter.
    Builder::from_env(Env::default().default_filter_or("info")).init();

    // Parse address used to bind exporter to.
    let addr_raw = "0.0.0.0:9185";
    let addr: SocketAddr = addr_raw.parse().expect("can not parse listen addr");

    // Start exporter
    let exporter = prometheus_exporter::start(addr).expect("can not start exporter");

    // Create metrics

    let score =
        register_gauge!("score", "will display score").expect("could not create gauge score");
    let dew_point = register_gauge!("dew_point", "will display dew_point")
        .expect("could not create gauge dew_point");
    let temp = register_gauge!("temp", "will display temp").expect("could not create gauge temp");
    let humid =
        register_gauge!("humid", "will display humid").expect("could not create gauge humid");
    let abs_humid = register_gauge!("abs_humid", "will display abs_humid")
        .expect("could not create gauge abs_humid");
    let co2 = register_gauge!("co2", "will display co2").expect("could not create gauge co2");
    let co2_est =
        register_gauge!("co2_est", "will display co2_est").expect("could not create gauge co2_est");
    let co2_est_baseline = register_gauge!("co2_est_baseline", "will display co2_est_baseline")
        .expect("could not create gauge co2_est_baseline");
    let voc = register_gauge!("voc", "will display voc").expect("could not create gauge voc");
    let voc_baseline = register_gauge!("voc_baseline", "will display voc_baseline")
        .expect("could not create gauge voc_baseline");
    let voc_h2_raw = register_gauge!("voc_h2_raw", "will display voc_h2_raw")
        .expect("could not create gauge voc_h2_raw");
    let voc_ethanol_raw = register_gauge!("voc_ethanol_raw", "will display voc_ethanol_raw")
        .expect("could not create gauge voc_ethanol_raw");
    let pm25 = register_gauge!("pm25", "will display pm25").expect("could not create gauge pm25");
    let pm10_est = register_gauge!("pm10_est", "will display pm10_est")
        .expect("could not create gauge pm10_est");

    // to test...
    // std::thread::spawn(move || {
    //     loop {
    //         std::thread::sleep(std::time::Duration::from_millis(1000));

    //         // Get metrics from exporter
    //         let body = reqwest::blocking::get(&format!("http://{}/metrics", addr_raw))
    //             .expect("can not get metrics from exporter")
    //             .text()
    //             .expect("can not body text from request");

    //         info!("Exporter metrics:\n{}", body);
    //     }
    // });
    
    let awair_local_url = env::var("AWAIR_LOCAL_URL").unwrap_or_else(|e| {
        panic!("could not find {}: {}", "AWAIR_LOCAL_URL", e)
    });
    if awair_local_url == "" {
        panic!("expected a non-empty value for env var AWAIR_LOCAL_URL")
    }

    info!("getting data from Awair Local API {}", &awair_local_url);

    loop {
        // Will block until a new request comes in.
        let _guard = exporter.wait_request();
        info!("Updating metrics");
        // Update metric
        let response = reqwest::blocking::get(&awair_local_url);
        match response {
            Ok(response) => {
                let response_body_result = response.json::<AwairLocalAirDataResponse>();
                match response_body_result {
                    Ok(response_body) => {
                        dbg!(&response_body);
                        score.set(response_body.score);
                        dew_point.set(response_body.dew_point);
                        temp.set(response_body.temp);
                        humid.set(response_body.humid);
                        abs_humid.set(response_body.abs_humid);
                        co2.set(response_body.co2);
                        co2_est.set(response_body.co2_est);
                        co2_est_baseline.set(response_body.co2_est_baseline);
                        voc.set(response_body.voc);
                        voc_baseline.set(response_body.voc_baseline);
                        voc_h2_raw.set(response_body.voc_h2_raw);
                        voc_ethanol_raw.set(response_body.voc_ethanol_raw);
                        pm25.set(response_body.pm25);
                        pm10_est.set(response_body.pm10_est);
                    }
                    Err(error) => {
                        dbg!(&error);
                    }
                }
            }
            Err(error) => {
                dbg!(&error);
            }
        }

        // info!("New random value: {}", new_value);

        // random.set(new_value);
    }
}
