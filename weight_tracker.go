// weight_tracker.go
package main

import (
	"encoding/csv"
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

const dataFile = "weight.json"

type Record struct {
	Date   string  `json:"date"`
	Weight float64 `json:"weight"`
	Photo  string  `json:"photo,omitempty"`
}

type Tracker struct {
	Records []Record `json:"records"`
}

func (t *Tracker) load() {
	data, err := os.ReadFile(dataFile)
	if err != nil {
		t.Records = []Record{}
		return
	}
	if err := json.Unmarshal(data, t); err != nil {
		t.Records = []Record{}
	}
	sort.Slice(t.Records, func(i, j int) bool { return t.Records[i].Date < t.Records[j].Date })
}

func (t *Tracker) save() {
	data, _ := json.MarshalIndent(t, "", "  ")
	os.WriteFile(dataFile, data, 0644)
}

func (t *Tracker) addRecord(weight float64, date, photo string) {
	if date == "" {
		date = time.Now().Format("2006-01-02")
	}
	rec := Record{Date: date, Weight: weight, Photo: photo}
	t.Records = append(t.Records, rec)
	t.save()
	fmt.Printf("\033[32mЗапись добавлена: %s - %.1f кг\033[0m", date, weight)
	if photo != "" {
		fmt.Printf(" (фото: %s)", photo)
	}
	fmt.Println()
}

func (t *Tracker) listRecords() {
	if len(t.Records) == 0 {
		fmt.Println("\033[33mНет записей.\033[0m")
		return
	}
	fmt.Println("\033[36mДата       | Вес (кг) | Фото\033[0m")
	for _, r := range t.Records {
		photo := r.Photo
		if photo == "" {
			photo = "-"
		}
		fmt.Printf("%s | %.1f | %s\n", r.Date, r.Weight, photo)
	}
}

func (t *Tracker) showChart() {
	if len(t.Records) == 0 {
		fmt.Println("\033[33mНет данных для графика.\033[0m")
		return
	}
	weights := make([]float64, len(t.Records))
	for i, r := range t.Records {
		weights[i] = r.Weight
	}
	minW, maxW := weights[0], weights[0]
	for _, w := range weights {
		if w < minW {
			minW = w
		}
		if w > maxW {
			maxW = w
		}
	}
	if maxW == minW {
		fmt.Println("Все значения одинаковы.")
		return
	}
	scale := 20.0 / (maxW - minW)
	fmt.Println("\033[36mГрафик изменения веса (кг):\033[0m")
	for _, r := range t.Records {
		barLen := int((r.Weight-minW)*scale) + 1
		bar := strings.Repeat("█", barLen)
		photo := r.Photo
		if photo != "" {
			photo = " " + photo
		}
		fmt.Printf("%s \033[32m%s\033[0m %.1f%s\n", r.Date, bar, r.Weight, photo)
	}
}

func (t *Tracker) showStats() {
	if len(t.Records) == 0 {
		fmt.Println("\033[33mНет данных.\033[0m")
		return
	}
	weights := make([]float64, len(t.Records))
	sum := 0.0
	for i, r := range t.Records {
		weights[i] = r.Weight
		sum += r.Weight
	}
	minW, maxW := weights[0], weights[0]
	for _, w := range weights {
		if w < minW {
			minW = w
		}
		if w > maxW {
			maxW = w
		}
	}
	avgW := sum / float64(len(weights))
	first := weights[0]
	last := weights[len(weights)-1]
	diff := last - first
	trend := "📈 растёт"
	if diff < 0 {
		trend = "📉 падает"
	} else if diff == 0 {
		trend = "➡️ стабилен"
	}
	fmt.Println("\033[36mСтатистика:\033[0m")
	fmt.Printf("  Минимальный: \033[32m%.1f\033[0m кг\n", minW)
	fmt.Printf("  Максимальный: \033[31m%.1f\033[0m кг\n", maxW)
	fmt.Printf("  Средний: \033[33m%.1f\033[0m кг\n", avgW)
	fmt.Printf("  Тренд: \033[35m%s\033[0m (%+.1f кг)\n", trend, diff)
}

func (t *Tracker) export(filename string) error {
	ext := strings.ToLower(filename[strings.LastIndex(filename, ".")+1:])
	var data []byte
	if ext == "json" {
		data, _ = json.MarshalIndent(t.Records, "", "  ")
	} else if ext == "csv" {
		f, err := os.Create(filename)
		if err != nil {
			return err
		}
		defer f.Close()
		w := csv.NewWriter(f)
		defer w.Flush()
		w.Write([]string{"date", "weight", "photo"})
		for _, r := range t.Records {
			w.Write([]string{r.Date, strconv.FormatFloat(r.Weight, 'f', 1, 64), r.Photo})
		}
		return nil
	} else {
		return fmt.Errorf("неподдерживаемый формат")
	}
	return os.WriteFile(filename, data, 0644)
}

func main() {
	var (
		add    float64
		date   string
		photo  string
		list   bool
		chart  bool
		stats  bool
		export string
	)
	flag.Float64Var(&add, "add", 0, "Добавить вес (кг)")
	flag.StringVar(&date, "date", "", "Дата (YYYY-MM-DD)")
	flag.StringVar(&photo, "photo", "", "Путь к фотографии")
	flag.BoolVar(&list, "list", false, "Показать записи")
	flag.BoolVar(&chart, "chart", false, "Показать график")
	flag.BoolVar(&stats, "stats", false, "Показать статистику")
	flag.StringVar(&export, "export", "", "Экспорт в файл")
	flag.Parse()

	tracker := &Tracker{}
	tracker.load()

	if add != 0 {
		tracker.addRecord(add, date, photo)
	} else if list {
		tracker.listRecords()
	} else if chart {
		tracker.showChart()
	} else if stats {
		tracker.showStats()
	} else if export != "" {
		if err := tracker.export(export); err != nil {
			fmt.Printf("\033[31mОшибка: %v\033[0m\n", err)
		} else {
			fmt.Printf("\033[32mЭкспортировано в %s\033[0m\n", export)
		}
	} else {
		fmt.Println("Используйте --help для справки.")
	}
}
