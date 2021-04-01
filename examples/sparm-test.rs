#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // Do some async work
        "return value"
    });

    // Do some other work
    println!("hello every one");

    let out = handle.await.unwrap();
    println!("GOT {}", out);
}