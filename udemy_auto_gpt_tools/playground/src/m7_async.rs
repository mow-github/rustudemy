use std::io::{Error, ErrorKind};
#[allow(dead_code)]
async fn my_async_call(url: &str) -> Result<serde_json::Value, reqwest::Error>{

    let response = reqwest::get(url)
    .await?
    .json::<serde_json::Value>()
    .await?;

    Ok(response)
}

#[allow(dead_code)]
async fn my_async_call_incl_error_handling(url: &str) -> Result<serde_json::Value, Error>{


    let response: reqwest::Response = reqwest::get(url)
    .await
    .map_err(|_| Error::new(ErrorKind::Other, "Could not get http data"))?;

    let json_response = response
    .json::<serde_json::Value>()
    .await
    .map_err(|_| Error::new(ErrorKind::Other, "Could not decode to json"))?;

    Ok(json_response)

    // Ok(serde_json::Value::from(0)) // dummy_rsp
}


#[cfg(test)]
mod test {
    use super::*;

    // cargo test tests_calls_async_fn -- --nocapture
    #[tokio::test]
    async fn tests_calls_async_fn() {
        dbg!("\n------------------------------------------->\n");

        let api_url = "http://jsonplaceholder.typicode.com/posts";
        let res = my_async_call(api_url).await;

        match res {
            Ok(r) => {
                dbg!(r)
            },
            Err(_) => {
                panic!("err bad request etc")
            }
        };
    }    

    // cargo test tests_my_async_call_incl_error_handling -- --nocapture
    #[tokio::test]
    async fn tests_my_async_call_incl_error_handling() {
        dbg!("\n------------------------------------------->\n");

        let api_url = "http://jsonplaceholder.typicode.com/posts";
        let res = my_async_call_incl_error_handling(api_url).await;

        match res {
            Ok(r) => {
                println!("IM OK !!!!!!!!!!!");
                dbg!(r)
            },
            Err(_) => {
                panic!("err bad request etc")
            }
        };
    }   

}
