use busa_http::read::line;
use criterion::{criterion_group, criterion_main, Criterion};
use smol::io::Cursor;
use std::time::Instant;

const METHOD: &[u8] = b"GET ";
const VERSION: &[u8] = b" HTTP/1.1\r\n";

async fn read_line(mut buf: Cursor<Vec<u8>>) {
  line::read_line(&mut buf).await.unwrap();
}

pub fn bench(buf: Cursor<Vec<u8>>) {
  smol::block_on(async {
    read_line(buf).await;
  });
}

fn benchmark(c: &mut Criterion) {
  c.bench_function("read_line", move |b| {
    b.iter_custom(|iters| {
      let buf: Vec<u8> = [
        METHOD,
        &(0..8000).map(|_| b'a').collect::<Vec<u8>>(),
        VERSION,
      ]
      .concat();

      let buf = Cursor::new(buf);
      let time = Instant::now();

      for _ in 0..iters {
        bench(buf.clone());
      }

      time.elapsed()
    })
  });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
