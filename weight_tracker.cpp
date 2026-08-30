// weight_tracker.cpp
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>
#include <ctime>
#include <iomanip>
#include <sstream>
#include <json/json.h> // using jsoncpp

using namespace std;

const string DATA_FILE = "weight.json";

struct Record {
    string date;
    double weight;
    string photo;
};

class Tracker {
private:
    vector<Record> records;

    void load() {
        ifstream ifs(DATA_FILE);
        if (!ifs) return;
        Json::Value root;
        ifs >> root;
        for (const auto& item : root) {
            Record r;
            r.date = item["date"].asString();
            r.weight = item["weight"].asDouble();
            r.photo = item.get("photo", "").asString();
            records.push_back(r);
        }
        sort(records.begin(), records.end(), [](const Record& a, const Record& b) {
            return a.date < b.date;
        });
    }

    void save() {
        Json::Value root(Json::arrayValue);
        for (const auto& r : records) {
            Json::Value item;
            item["date"] = r.date;
            item["weight"] = r.weight;
            if (!r.photo.empty()) item["photo"] = r.photo;
            root.append(item);
        }
        ofstream ofs(DATA_FILE);
        ofs << root.toStyledString();
    }

    string today() {
        time_t t = time(nullptr);
        tm* now = localtime(&t);
        char buf[11];
        strftime(buf, sizeof(buf), "%Y-%m-%d", now);
        return string(buf);
    }

public:
    Tracker() { load(); }

    void addRecord(double weight, const string& date, const string& photo) {
        string d = date.empty() ? today() : date;
        records.push_back({d, weight, photo});
        sort(records.begin(), records.end(), [](const Record& a, const Record& b) {
            return a.date < b.date;
        });
        save();
        cout << "\033[32mЗапись добавлена: " << d << " - " << weight << " кг";
        if (!photo.empty()) cout << " (фото: " << photo << ")";
        cout << "\033[0m" << endl;
    }

    void listRecords() {
        if (records.empty()) {
            cout << "\033[33mНет записей.\033[0m" << endl;
            return;
        }
        cout << "\033[36mДата       | Вес (кг) | Фото\033[0m" << endl;
        for (const auto& r : records) {
            string photo = r.photo.empty() ? "-" : r.photo;
            cout << r.date << " | " << fixed << setprecision(1) << r.weight << " | " << photo << endl;
        }
    }

    void showChart() {
        if (records.empty()) {
            cout << "\033[33mНет данных для графика.\033[0m" << endl;
            return;
        }
        double minW = records[0].weight, maxW = records[0].weight;
        for (const auto& r : records) {
            if (r.weight < minW) minW = r.weight;
            if (r.weight > maxW) maxW = r.weight;
        }
        if (maxW == minW) {
            cout << "Все значения одинаковы." << endl;
            return;
        }
        double scale = 20.0 / (maxW - minW);
        cout << "\033[36mГрафик изменения веса (кг):\033[0m" << endl;
        for (const auto& r : records) {
            int barLen = (int)((r.weight - minW) * scale) + 1;
            string bar(barLen, '█');
            string photo = r.photo.empty() ? "" : " " + r.photo;
            cout << r.date << " \033[32m" << bar << "\033[0m " << fixed << setprecision(1) << r.weight << photo << endl;
        }
    }

    void showStats() {
        if (records.empty()) {
            cout << "\033[33mНет данных.\033[0m" << endl;
            return;
        }
        double minW = records[0].weight, maxW = records[0].weight, sum = 0;
        for (const auto& r : records) {
            if (r.weight < minW) minW = r.weight;
            if (r.weight > maxW) maxW = r.weight;
            sum += r.weight;
        }
        double avgW = sum / records.size();
        double first = records.front().weight;
        double last = records.back().weight;
        double diff = last - first;
        string trend = diff > 0 ? "📈 растёт" : diff < 0 ? "📉 падает" : "➡️ стабилен";
        cout << "\033[36mСтатистика:\033[0m" << endl;
        cout << "  Минимальный: \033[32m" << fixed << setprecision(1) << minW << "\033[0m кг" << endl;
        cout << "  Максимальный: \033[31m" << maxW << "\033[0m кг" << endl;
        cout << "  Средний: \033[33m" << avgW << "\033[0m кг" << endl;
        cout << "  Тренд: \033[35m" << trend << "\033[0m (" << showpos << diff << " кг)" << endl;
    }

    void exportData(const string& filename) {
        string ext = filename.substr(filename.find_last_of('.') + 1);
        if (ext == "json") {
            ofstream ofs(filename);
            Json::Value root(Json::arrayValue);
            for (const auto& r : records) {
                Json::Value item;
                item["date"] = r.date;
                item["weight"] = r.weight;
                if (!r.photo.empty()) item["photo"] = r.photo;
                root.append(item);
            }
            ofs << root.toStyledString();
        } else if (ext == "csv") {
            ofstream ofs(filename);
            ofs << "date,weight,photo\n";
            for (const auto& r : records) {
                ofs << r.date << "," << fixed << setprecision(1) << r.weight << "," << r.photo << "\n";
            }
        } else {
            cerr << "\033[31mНеподдерживаемый формат. Используйте .json или .csv\033[0m" << endl;
            return;
        }
        cout << "\033[32mЭкспортировано в " << filename << "\033[0m" << endl;
    }
};

int main(int argc, char* argv[]) {
    double add = 0;
    string date, photo, exportFile;
    bool list = false, chart = false, stats = false;

    for (int i = 1; i < argc; ++i) {
        string arg = argv[i];
        if (arg == "--add" && i+1 < argc) add = stod(argv[++i]);
        else if (arg == "--date" && i+1 < argc) date = argv[++i];
        else if (arg == "--photo" && i+1 < argc) photo = argv[++i];
        else if (arg == "--list") list = true;
        else if (arg == "--chart") chart = true;
        else if (arg == "--stats") stats = true;
        else if (arg == "--export" && i+1 < argc) exportFile = argv[++i];
    }

    Tracker tracker;
    if (add != 0) {
        tracker.addRecord(add, date, photo);
    } else if (list) {
        tracker.listRecords();
    } else if (chart) {
        tracker.showChart();
    } else if (stats) {
        tracker.showStats();
    } else if (!exportFile.empty()) {
        tracker.exportData(exportFile);
    } else {
        cout << "Используйте --help для справки." << endl;
    }
    return 0;
}
