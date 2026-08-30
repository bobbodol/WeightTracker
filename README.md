# Трекер веса (фотографии)

Многоязычное консольное приложение для отслеживания динамики веса с возможностью прикрепления фотографий.  
Позволяет добавлять записи с весом и датой, привязывать изображения, просматривать прогресс, строить ASCII-график, рассчитывать статистику и экспортировать данные.

## Особенности
- Добавление записей (дата, вес, путь к фотографии).
- Просмотр всех записей с отображением даты, веса и ссылки на фото.
- Визуализация изменения веса в виде ASCII-графика (столбчатая диаграмма).
- Расчёт статистики: минимальный, максимальный, средний вес, общая динамика.
- Экспорт данных в JSON и CSV.
- Хранение данных в локальном JSON-файле.
- Цветной вывод в терминале (где поддерживается).
- Поддержка аргументов командной строки для всех операций.

## Установка и запуск
Для каждого языка требуются соответствующие инструменты и зависимости (указаны ниже).

### Запуск на разных языках

1. **Python**  
   Установка: `pip install colorama` (опционально).  
   Запуск: `python weight_tracker.py --add 75.5 --photo photo.jpg`

2. **JavaScript (Node.js)**  
   Установка: `npm install commander chalk`  
   Запуск: `node weight_tracker.js --add 75.5 --photo photo.jpg`

3. **Go**  
   Запуск: `go run weight_tracker.go --add 75.5 --photo photo.jpg`

4. **Rust**  
   Сборка: `cargo build --release`  
   Запуск: `cargo run -- --add 75.5 --photo photo.jpg`

5. **Java**  
   Сборка: `javac -cp gson.jar WeightTracker.java`  
   Запуск: `java -cp .;gson.jar WeightTracker --add 75.5 --photo photo.jpg`

6. **C# (.NET Core)**  
   Установка: `dotnet add package Newtonsoft.Json`  
   Запуск: `dotnet run -- --add 75.5 --photo photo.jpg`

7. **C++ (Linux)**  
   Сборка: `g++ -std=c++11 -o weight_tracker weight_tracker.cpp -ljsoncpp`  
   Запуск: `./weight_tracker --add 75.5 --photo photo.jpg`

8. **Kotlin (JVM)**  
   Сборка: `kotlinc -cp gson.jar WeightTracker.kt`  
   Запуск: `kotlin -cp .;gson.jar WeightTrackerKt --add 75.5 --photo photo.jpg`

## Использование

Общие аргументы командной строки (везде, где поддерживается):

- `--add <вес>` – добавить запись о весе (сегодняшняя дата, опционально `--date` и `--photo`).
- `--date <YYYY-MM-DD>` – указать дату для добавления.
- `--photo <путь>` – путь к файлу фотографии для записи.
- `--list` – показать все записи.
- `--chart` – показать ASCII-график изменения веса.
- `--stats` – показать статистику.
- `--export <файл>` – экспортировать данные в JSON или CSV (по расширению).
- `--help` – справка.

Пример (Python):
```bash
python weight_tracker.py --add 75.5 --photo before.jpg
python weight_tracker.py --list
python weight_tracker.py --chart
python weight_tracker.py --export data.json
Структура репозитория
text
/
├── README.md
├── weight_tracker.py
├── weight_tracker.js
├── weight_tracker.go
├── weight_tracker.rs
├── WeightTracker.java
├── WeightTracker.cs
├── weight_tracker.cpp
└── WeightTracker.kt
Лицензия
MIT
