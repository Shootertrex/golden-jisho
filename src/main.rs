use std::env;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Response {
    data: Vec<Data>
}

#[derive(Deserialize, Debug)]
struct Data {
    // japanese: Vec<HashMap<String, String>>
    japanese: Vec<Japanese>,
    senses: Vec<Senses>,
}

#[derive(Deserialize, Debug)]
struct Japanese {
    word: Option<String>,
    reading: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Senses {
    see_also: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let expression = args[1].to_owned();
    let EMPTY_TEXT = "";

    // Build the client using the builder pattern
    let client = reqwest::Client::builder()
        .build()?;

    let request_url = format!("https://jisho.org/api/v1/search/words?keyword={}", expression);
    // Perform the actual execution of the network request
    let res = client
        .get(request_url)
        .send()
        .await?;

    // Parse the response body as Json in this case
    let response = res
        .json::<Response>()
        .await?;

    if response.data.len() == 0 {
        return Ok(());
    }

    let japanese = &response.data[0].japanese[0];

    let data = format!(r#"<html>
    <head>
        <meta charset="UTF-8">
        <title></title>
    </head>
    <body>
        <h1>
            <span class="text">
                <a href="gdlookup://localhost/{word}">
                    <ruby>{word}<rt>{reading}</rt></ruby>
                </a>
            </span>
        </h1> {see_also}
        <div>
            Expression: {expression}
        </div>
    </body>
</html>"#, word=japanese.word.as_ref().unwrap_or(&EMPTY_TEXT.to_string()),
        reading=japanese.reading.as_ref().unwrap_or(&EMPTY_TEXT.to_string()),
        expression=expression,
        see_also=create_see_also(&response));
    println!("{}", data);
    return Ok(());
}

fn create_see_also(response: &Response) -> String {
    if response.data.len() == 0 
        || response.data[0].senses.len() == 0
        || response.data[0].senses[0].see_also.len() == 0 {
        return String::new();
    }

    return format!(r#"
        <div>
            See also: <a href="gdlookup://localhost/{see_also}">{see_also}</a>
        </div>"#, see_also=&response.data[0].senses[0].see_also[0]);

}