
#!/usr/bin/env python3
# weight_tracker.py
import argparse
import json
import csv
import os
import sys
from datetime import datetime, timedelta
from colorama import init, Fore, Style

init(autoreset=True)

DATA_FILE = "weight.json"

class WeightTracker:
    def __init__(self):
        self.records = []
        self.load()

    def load(self):
        if not os.path.exists(DATA_FILE):
            return
        try:
            with open(DATA_FILE, 'r') as f:
                self.records = json.load(f)
        except:
            self.records = []

    def save(self):
        with open(DATA_FILE, 'w') as f:
            json.dump(self.records, f, indent=2)

    def add_record(self, weight, date=None, photo=None):
        if date is None:
            date = datetime.now().strftime("%Y-%m-%d")
        record = {"date": date, "weight": weight}
        if photo:
            record["photo"] = photo
        self.records.append(record)
        self.records.sort(key=lambda x: x["date"])
        self.save()
        print(Fore.GREEN + f"Запись добавлена: {date} - {weight} кг" + (f" (фото: {photo})" if photo else ""))

    def list_records(self):
        if not self.records:
            print(Fore.YELLOW + "Нет записей.")
            return
        print(Fore.CYAN + "Дата       | Вес (кг) | Фото")
        for r in self.records:
            photo = r.get("photo", "-")
            print(f"{r['date']} | {r['weight']:.1f} | {photo}")

    def show_chart(self):
        if not self.records:
            print(Fore.YELLOW + "Нет данных для графика.")
            return
        weights = [r["weight"] for r in self.records]
        min_w = min(weights)
        max_w = max(weights)
        if max_w == min_w:
            print("Все значения одинаковы.")
            return
        scale = 20 / (max_w - min_w)
        print(Fore.CYAN + "График изменения веса (кг):")
        for r in self.records:
            bar_len = int((r["weight"] - min_w) * scale) + 1
            bar = "█" * bar_len
            photo = r.get("photo", "")
            print(f"{r['date']} {Fore.GREEN}{bar} {r['weight']:.1f}  {photo}")

    def show_stats(self):
        if not self.records:
            print(Fore.YELLOW + "Нет данных.")
            return
        weights = [r["weight"] for r in self.records]
        min_w = min(weights)
        max_w = max(weights)
        avg_w = sum(weights) / len(weights)
        first = weights[0]
        last = weights[-1]
        diff = last - first
        trend = "📈 растёт" if diff > 0 else "📉 падает" if diff < 0 else "➡️ стабилен"
        print(Fore.CYAN + "Статистика:")
        print(f"  Минимальный: {Fore.GREEN}{min_w:.1f} кг")
        print(f"  Максимальный: {Fore.RED}{max_w:.1f} кг")
        print(f"  Средний: {Fore.YELLOW}{avg_w:.1f} кг")
        print(f"  Тренд: {Fore.MAGENTA}{trend} ({diff:+.1f} кг)")

    def export(self, filename):
        ext = os.path.splitext(filename)[1].lower()
        if ext == '.json':
            with open(filename, 'w') as f:
                json.dump(self.records, f, indent=2)
        elif ext == '.csv':
            with open(filename, 'w', newline='') as f:
                writer = csv.writer(f)
                writer.writerow(["date", "weight", "photo"])
                for r in self.records:
                    writer.writerow([r["date"], r["weight"], r.get("photo", "")])
        else:
            print(Fore.RED + "Неподдерживаемый формат. Используйте .json или .csv")
            return
        print(Fore.GREEN + f"Экспортировано в {filename}")

def main():
    parser = argparse.ArgumentParser(description="Трекер веса с фотографиями")
    parser.add_argument("--add", type=float, help="Добавить вес (кг)")
    parser.add_argument("--date", help="Дата (YYYY-MM-DD)")
    parser.add_argument("--photo", help="Путь к фотографии")
    parser.add_argument("--list", action="store_true", help="Показать записи")
    parser.add_argument("--chart", action="store_true", help="Показать график")
    parser.add_argument("--stats", action="store_true", help="Показать статистику")
    parser.add_argument("--export", help="Экспорт в файл (JSON/CSV)")
    args = parser.parse_args()

    tracker = WeightTracker()
    if args.add is not None:
        tracker.add_record(args.add, args.date, args.photo)
    elif args.list:
        tracker.list_records()
    elif args.chart:
        tracker.show_chart()
    elif args.stats:
        tracker.show_stats()
    elif args.export:
        tracker.export(args.export)
    else:
        parser.print_help()

if __name__ == "__main__":
    main()
