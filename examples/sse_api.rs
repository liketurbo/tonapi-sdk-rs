use simple_logger::SimpleLogger;
use tonapi::stream_api::{
    MempoolStreamParams, SseApi, SseApiConfig, TracesStreamParams, TransactionsStreamParams,
};

async fn subscribe_to_transactions(sse: &SseApi) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = sse.transactions_stream(TransactionsStreamParams {
        accounts: Some(vec![
            "-1:5555555555555555555555555555555555555555555555555555555555555555".to_string(),
        ]),
        operations: None,
    });

    while let Ok(evt) = stream.next().await {
        if let Some(evt) = evt {
            println!("Event: {}", evt.tx_hash);
        } else {
            // Stream ended
            break;
        }
    }

    Ok(())
}

async fn subscribe_to_traces(sse: &SseApi) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = sse.traces_stream(TracesStreamParams {
        accounts: Some(vec![
            "-1:5555555555555555555555555555555555555555555555555555555555555555".to_string(),
        ]),
    });

    println!("{:?}", stream.next().await.unwrap());

    while let Ok(evt) = stream.next().await {
        if let Some(evt) = evt {
            println!("Event: {}", evt.hash);
        } else {
            // Stream ended
            break;
        }
    }

    Ok(())
}

async fn subscribe_to_mempool(sse: &SseApi) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = sse.mempool_stream(MempoolStreamParams {
        accounts: Some(vec![
            "-1:5555555555555555555555555555555555555555555555555555555555555555".to_string(),
        ]),
    });

    stream.next().await.unwrap();

    while let Ok(evt) = stream.next().await {
        if let Some(evt) = evt {
            println!("Boc: {}", evt.boc);
        } else {
            // Stream ended
            break;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().init().expect("logging init");
    let sse_api = SseApi::new(SseApiConfig { auth_token: None });

    let _ = subscribe_to_transactions(&sse_api).await;
    let _ = subscribe_to_traces(&sse_api).await;
    let _ = subscribe_to_mempool(&sse_api).await;

    Ok(())
}
