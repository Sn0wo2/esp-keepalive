use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::io::{stdout, AsyncWriteExt};
use tokio::sync::mpsc;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Clone)]
pub struct AsyncStdoutWriter {
    pub sender: mpsc::UnboundedSender<Vec<u8>>,
    pub count: Arc<AtomicUsize>,
}

impl io::Write for AsyncStdoutWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let _ = self.sender.send(buf.to_vec());
        self.count.fetch_add(1, Ordering::Relaxed);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for AsyncStdoutWriter {
    type Writer = Self;

    fn make_writer(&self) -> Self::Writer {
        self.clone()
    }
}

pub fn init_logging() -> AsyncStdoutWriter {
    let (tx, mut rx) = mpsc::unbounded_channel::<Vec<u8>>();
    let count = Arc::new(AtomicUsize::new(0));

    let writer = AsyncStdoutWriter {
        sender: tx,
        count: count.clone(),
    };

    let task_count = count.clone();
    tokio::spawn(async move {
        let mut stdout = stdout();
        while let Some(data) = rx.recv().await {
            let _ = stdout.write_all(&data).await;

            task_count.fetch_sub(1, Ordering::Relaxed);
        }
    });

    let writer_for_tracing = writer.clone();

    tracing_subscriber::registry()
        .with(fmt::layer()
            .with_writer(writer_for_tracing)
            .with_ansi(true)
            .with_level(true)
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_thread_names(true))
        .with(EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("debug")))
        .init();

    writer
}
