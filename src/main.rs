mod qlib;

use crate::qlib::{qserve, QServeOptions};

#[tokio::main]
async fn main() {
    let config = QServeOptions::get_config();

    qserve(config).await;
}
