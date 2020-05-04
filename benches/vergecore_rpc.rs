use criterion::{criterion_group, criterion_main, Criterion};

use verge_hashes::{hex::FromHex, sha256d::Hash as Sha256dHash};
use vergecore_rpc::RpcApi;

fn get_rpc() -> vergecore_rpc::Client {
    vergecore_rpc::Client::new(
        "http://localhost:20102".into(),
        vergecore_rpc::Auth::UserPass("user".into(), "magicpassword".into()),
    )
    .expect("rpc client creation")
}

const BLOCK_HASH: &str = "656f93a73870b4d974c20cf7d43de4fd6cc02bf9b7693fb967bcb3da6becb4bb";

fn get_block(c: &mut Criterion) {
    c.bench_function("getblock", |b| {
        let rpc = get_rpc();
        let hash = Sha256dHash::from_hex(BLOCK_HASH).unwrap();

        b.iter(|| rpc.get_block(&hash).unwrap())
    });
    c.bench_function("getblock_verbose", |b| {
        let rpc = get_rpc();
        let hash = Sha256dHash::from_hex(BLOCK_HASH).unwrap();

        b.iter(|| rpc.get_block(&hash).unwrap())
    });
    c.bench_function("getrawtransaction", |b| {
        let rpc = get_rpc();
        let hash = Sha256dHash::from_hex(
            "55d34392ca0bacb471ea57d7d5eabcdb0fcaaa6d611cbf8586f35c2e45d3da7c",
        )
        .unwrap();

        b.iter(|| rpc.get_raw_transaction(&hash, None).unwrap())
    });
}

criterion_group!(benches, get_block);
criterion_main!(benches);
