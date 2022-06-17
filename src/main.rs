extern crate fs_extra;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

fn main() {
    println!("Hello, world!");

    let email = Message::builder()
        .from("admin <admin@chenjiaming.org>".parse().unwrap())
        .reply_to("aircjm <aircjm@gmail.com>".parse().unwrap())
        .to("aircjm <aircjm@gmail.com>".parse().unwrap())
        .subject("Happy new year")
        .body(String::from("Be happy!"))
        .unwrap();

    let creds = Credentials::new(
        "admin@chenjiaming.org".to_string(),
        "mgErTxRFBsSndYIY".to_string(),
    );

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.feishu.cn")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}


