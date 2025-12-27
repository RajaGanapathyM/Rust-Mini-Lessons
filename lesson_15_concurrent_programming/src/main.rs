use tokio::sync::mpsc;
use std::time::Instant;

#[derive(Debug)]
enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug)]
struct LogEntry{
    timestamp: Instant,
    severity: Severity,
    message: String,
}

impl std::fmt::Display for LogEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] {:?}: {}", self.timestamp, self.severity, self.message)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx,mut rx)=mpsc::unbounded_channel::<LogEntry>();

    for i in 0..3{
        let tx_clone=tx.clone();

        tokio::spawn(async move{
            for j in 0..5{
                let entry= LogEntry{
                    timestamp: Instant::now(),
                    severity: match j % 3 {
                        0 => Severity::Info,
                        1 => Severity::Warning,
                        _ => Severity::Error,
                    },
                    message: format!("Producer {} log {}",i,j),
                };
                tx_clone.send(entry).unwrap();

                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        });

    }

    drop(tx); // Close the sender to signal no more messages will be sent

    let mut counts=std::collections::HashMap::new();
    while let Some(log)=rx.recv().await{
        let count=counts.entry(format!("{:?}",log.severity)).or_insert(0);
        *count+=1;
        println!("Received: {}",log);
    }
    

    println!("Log summary: {:?}",counts);
    Ok(())
}