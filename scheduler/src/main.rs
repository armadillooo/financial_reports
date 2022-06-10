use scheduler::all_company;

#[tokio::main]
async fn main() {
    all_company().await.unwrap();
}