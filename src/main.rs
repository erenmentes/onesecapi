use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use rand::seq::IndexedRandom;

#[get("/one")]
async fn onesec() -> impl Responder {
    let events = vec![
        "5.4 million cells are dying in the human body.",
        "Light is traveling around the Earth 7.5 times.",
        "$2,400 worth of online shopping is being done.",
        "At least 100,000 financial transactions are occurring in the global economy.",
        "Approximately 300 chickens are being slaughtered per second for human consumption.",
        "24 tons of glaciers are melting.",
        "More than 400,000 bees are collecting pollen from flowers.",
        "3 million Google searches are being made.",
        "5,000 TikTok videos are being watched.",
        "4 babies are being born.",
        "1.5 million tons of water is evaporating.",
        "9,000 tweets are being sent.",
        "1.5 million barrels of oil are being consumed.",
        "Bill Gates is earning $250.",
        "3 million emails are being sent.",
        "Our planet is rotating 460 meters on its own axis and is moving 30 kilometers around the Sun.",
        "3 sharks are being killed by humans in the fishing industry.",
        "2 million WhatsApp messages, including photos and voice messages, are being sent.",
        "30 trees are being cut down.",
        "Approximately 2 people are losing their lives.",
        "1,500 barrels of oil are being produced.",
        "The Voyager 2 satellite is traveling 926 kilometers in space.",
        "1.5 million tons of water is evaporating.",
        "More than 80 people are signing up for a social media platform.",
        "More than 5,000 ants are carrying food to their nest.",
        "At least 100 lightning strikes are occurring, producing enough energy to charge your phone for hundreds of years.",
        "A bee is flapping its wings 270 times.",
    ];
    let mut random = rand::rng();

    let chosen = events
        .choose(&mut random)
        .unwrap_or(&"An unknown error occured.");

    HttpResponse::Ok().body(*chosen)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
        .wrap(cors)
        .service(onesec)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
