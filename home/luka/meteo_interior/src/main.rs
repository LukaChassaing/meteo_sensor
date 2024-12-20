use rppal::gpio::{Gpio, IoPin, Level, Mode, PullUpDown};
use std::{thread, time::{Duration, Instant}};
use std::error::Error;
use serde::{Serialize, Deserialize};
use reqwest::Client;
use std::env;
use dotenv::dotenv;
use tokio;

// Structure pour l'envoi des données
#[derive(Serialize, Debug)]
struct Measurement {
    temperature: Temperature,
    humidity: Humidity,
    location: String,
}

#[derive(Serialize, Debug)]
struct Temperature {
    value: f32,
    unit: String,
}

#[derive(Serialize, Debug)]
struct Humidity {
    value: f32,
    unit: String,
}

struct Dht11 {
    pin: u8,
    gpio: rppal::gpio::Gpio,
    client: Client,
    server_url: String,
    location: String,
}

impl Dht11 {
    fn new(pin: u8, server_url: String, location: String) -> Result<Self, Box<dyn Error>> {
        Ok(Dht11 {
            pin,
            gpio: Gpio::new()?,
            client: Client::new(),
            server_url,
            location,
        })
    }

    fn wait_level(&self, pin: &IoPin, level: Level, timeout_micros: u64) -> Result<u64, Box<dyn Error>> {
        let start = Instant::now();
        while pin.read() == level {
            if start.elapsed() > Duration::from_micros(timeout_micros) {
                return Err(format!("Timeout en attente de {:?}", level).into());
            }
        }
        Ok(start.elapsed().as_micros() as u64)
    }

    async fn send_measurement(&self, temperature: f32, humidity: f32) -> Result<(), Box<dyn Error>> {
        let measurement = Measurement {
            temperature: Temperature {
                value: temperature,
                unit: "°C".to_string(),
            },
            humidity: Humidity {
                value: humidity,
                unit: "%".to_string(),
            },
            location: self.location.clone(),
        };

        let response = self.client
            .post(format!("{}/push-measures", self.server_url))
            .json(&measurement)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Erreur serveur: {}", response.status()).into());
        }

        Ok(())
    }

    fn read(&self) -> Result<(f32, f32), Box<dyn Error>> {
        let mut data = [0u8; 5];
        
        let mut pin = self.gpio.get(self.pin)?.into_io(Mode::Output);
        pin.set_pullupdown(PullUpDown::PullUp);
        
        pin.set_low();
        thread::sleep(Duration::from_millis(20));
        pin.set_high();
        
        pin.set_mode(Mode::Input);
        
        self.wait_level(&pin, Level::High, 100)?;
        self.wait_level(&pin, Level::Low, 100)?;
        self.wait_level(&pin, Level::High, 100)?;

        let mut bit;
        for i in 0..40 {
            self.wait_level(&pin, Level::Low, 100)?;
            let high_duration = self.wait_level(&pin, Level::High, 100)?;
            
            bit = if high_duration > 40 { 1 } else { 0 };
            data[i/8] |= bit << (7 - (i % 8));
        }

        let checksum = (data[0] + data[1] + data[2] + data[3]) & 0xFF;
        if data[4] != checksum {
            if (data[4] as i16 - checksum as i16).abs() <= 1 {
                eprintln!("Warning: Checksum légèrement incorrect mais données acceptées");
            } else {
                return Err(format!("Erreur de checksum. Calculé: {}, Reçu: {}", checksum, data[4]).into());
            }
        }

        let humidity = data[0] as f32;
        let temperature = data[2] as f32;

        Ok((temperature, humidity))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Chargement des variables d'environnement
    dotenv().ok();

    let server_url = env::var("SERVER_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());
    let location = env::var("SENSOR_LOCATION")
        .unwrap_or_else(|_| "interior".to_string());
    let read_interval = env::var("READ_INTERVAL_SECS")
        .unwrap_or_else(|_| "60".to_string())
        .parse::<u64>()?;

    println!("Démarrage du programme de lecture DHT11...");
    println!("URL du serveur: {}", server_url);
    println!("Location: {}", location);
    println!("Intervalle de lecture: {}s", read_interval);
    
    let dht11 = Dht11::new(4, server_url, location)?;
    let mut consecutive_errors = 0;

    loop {
        match dht11.read() {
            Ok((temperature, humidity)) => {
                println!("Lecture réussie !");
                println!("Température: {:.1}°C", temperature);
                println!("Humidité: {:.1}%", humidity);

                match dht11.send_measurement(temperature, humidity).await {
                    Ok(_) => {
                        println!("Données envoyées avec succès au serveur");
                        consecutive_errors = 0;
                    },
                    Err(e) => {
                        eprintln!("Erreur lors de l'envoi au serveur: {}", e);
                        consecutive_errors += 1;
                    }
                }
            },
            Err(e) => {
                eprintln!("Erreur de lecture: {}", e);
                consecutive_errors += 1;
            }
        }

        // Gestion des erreurs consécutives
        if consecutive_errors >= 5 {
            eprintln!("Trop d'erreurs consécutives, attente plus longue...");
            thread::sleep(Duration::from_secs(read_interval * 2));
            consecutive_errors = 0;
        } else {
            thread::sleep(Duration::from_secs(read_interval));
        }
    }
}