// weight_tracker.rs
use chrono::Local;
use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::Write;
use colored::*;

const DATA_FILE: &str = "weight.json";

#[derive(Serialize, Deserialize, Clone)]
struct Record {
    date: String,
    weight: f64,
    photo: Option<String>,
}

struct Tracker {
    records: Vec<Record>,
}

impl Tracker {
    fn new() -> Self {
        let mut t = Tracker { records: Vec::new() };
        t.load();
        t
    }

    fn load(&mut self) {
        if let Ok(data) = fs::read_to_string(DATA_FILE) {
            if let Ok(records) = serde_json::from_str(&data) {
                self.records = records;
                self.records.sort_by(|a, b| a.date.cmp(&b.date));
                return;
            }
        }
        self.records = Vec::new();
    }

    fn save(&self) {
        let json = serde_json::to_string_pretty(&self.records).unwrap();
        fs::write(DATA_FILE, json).unwrap();
    }

    fn add_record(&mut self, weight: f64, date: Option<&str>, photo: Option<&str>) {
        let date_str = date.unwrap_or(&Local::now().format("%Y-%m-%d").to_string()).to_string();
        let record = Record {
            date: date_str.clone(),
            weight,
            photo: photo.map(|s| s.to_string()),
        };
        self.records.push(record);
        self.records.sort_by(|a, b| a.date.cmp(&b.date));
        self.save();
        print!("{}", format!("Запись добавлена: {} - {:.1} кг", date_str, weight).green());
        if let Some(p) = photo {
            print!(" (фото: {})", p);
        }
        println!();
    }

    fn list_records(&self) {
        if self.records.is_empty() {
            println!("{}", "Нет записей.".yellow());
            return;
        }
        println!("{}", "Дата       | Вес (кг) | Фото".cyan());
        for r in &self.records {
            let photo = r.photo.as_deref().unwrap_or("-");
            println!("{} | {:.1} | {}", r.date, r.weight, photo);
        }
    }

    fn show_chart(&self) {
        if self.records.is_empty() {
            println!("{}", "Нет данных для графика.".yellow());
            return;
        }
        let weights: Vec<f64> = self.records.iter().map(|r| r.weight).collect();
        let min_w = weights.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_w = weights.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        if (max_w - min_w).abs() < 1e-9 {
            println!("Все значения одинаковы.");
            return;
        }
        let scale = 20.0 / (max_w - min_w);
        println!("{}", "График изменения веса (кг):".cyan());
        for r in &self.records {
            let bar_len = ((r.weight - min_w) * scale) as usize + 1;
            let bar = "█".repeat(bar_len);
            let photo = r.photo.as_deref().unwrap_or("");
            println!("{} {:.1}{}", format!("{} {}", r.date, bar.green()), r.weight, if !photo.is_empty() { format!(" {}", photo) } else { "".to_string() });
        }
    }

    fn show_stats(&self) {
        if self.records.is_empty() {
            println!("{}", "Нет данных.".yellow());
            return;
        }
        let weights: Vec<f64> = self.records.iter().map(|r| r.weight).collect();
        let min_w = weights.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_w = weights.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let sum: f64 = weights.iter().sum();
        let avg_w = sum / weights.len() as f64;
        let first = weights[0];
        let last = weights[weights.len()-1];
        let diff = last - first;
        let trend = if diff > 0.0 { "📈 растёт" } else if diff < 0.0 { "📉 падает" } else { "➡️ стабилен" };
        println!("{}", "Статистика:".cyan());
        println!("  Минимальный: {:.1} кг", min_w.to_string().green());
        println!("  Максимальный: {:.1} кг", max_w.to_string().red());
        println!("  Средний: {:.1} кг", avg_w.to_string().yellow());
        println!("  Тренд: {} ({:+.1} кг)", trend.magenta(), diff);
    }

    fn export(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let ext = filename.split('.').last().unwrap_or("").to_lowercase();
        if ext == "json" {
            let json = serde_json::to_string_pretty(&self.records)?;
            fs::write(filename, json)?;
        } else if ext == "csv" {
            let mut wtr = csv::Writer::from_path(filename)?;
            wtr.write_record(&["date", "weight", "photo"])?;
            for r in &self.records {
                wtr.write_record(&[&r.date, &r.weight.to_string(), r.photo.as_deref().unwrap_or("")])?;
            }
            wtr.flush()?;
        } else {
            return Err("Неподдерживаемый формат".into());
        }
        println!("{}", format!("Экспортировано в {}", filename).green());
        Ok(())
    }
}

fn main() {
    let matches = App::new("Weight Tracker with Photos")
        .arg(Arg::with_name("add").long("add").takes_value(true).help("Добавить вес (кг)"))
        .arg(Arg::with_name("date").long("date").takes_value(true).help("Дата (YYYY-MM-DD)"))
        .arg(Arg::with_name("photo").long("photo").takes_value(true).help("Путь к фотографии"))
        .arg(Arg::with_name("list").long("list").help("Показать записи"))
        .arg(Arg::with_name("chart").long("chart").help("Показать график"))
        .arg(Arg::with_name("stats").long("stats").help("Показать статистику"))
        .arg(Arg::with_name("export").long("export").takes_value(true).help("Экспорт в файл"))
        .get_matches();

    let mut tracker = Tracker::new();

    if let Some(weight_str) = matches.value_of("add") {
        let weight: f64 = weight_str.parse().expect("Неверный вес");
        tracker.add_record(weight, matches.value_of("date"), matches.value_of("photo"));
    } else if matches.is_present("list") {
        tracker.list_records();
    } else if matches.is_present("chart") {
        tracker.show_chart();
    } else if matches.is_present("stats") {
        tracker.show_stats();
    } else if let Some(file) = matches.value_of("export") {
        if let Err(e) = tracker.export(file) {
            eprintln!("{}", format!("Ошибка: {}", e).red());
        }
    } else {
        println!("Используйте --help для справки.");
    }
}
