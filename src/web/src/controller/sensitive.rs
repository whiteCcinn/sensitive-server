use tide::Request;

pub async fn filter(_req: Request<()>) -> String {
    String::from("hello")
}