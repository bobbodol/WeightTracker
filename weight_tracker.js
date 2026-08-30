#!/usr/bin/env node
// weight_tracker.js
const { program } = require('commander');
const fs = require('fs');
const chalk = require('chalk');

const DATA_FILE = 'weight.json';

class WeightTracker {
    constructor() {
        this.records = [];
        this.load();
    }

    load() {
        try {
            if (fs.existsSync(DATA_FILE)) {
                this.records = JSON.parse(fs.readFileSync(DATA_FILE, 'utf8'));
            }
        } catch (e) {
            this.records = [];
        }
        this.records.sort((a, b) => a.date.localeCompare(b.date));
    }

    save() {
        fs.writeFileSync(DATA_FILE, JSON.stringify(this.records, null, 2));
    }

    addRecord(weight, date, photo) {
        if (!date) date = new Date().toISOString().split('T')[0];
        const record = { date, weight };
        if (photo) record.photo = photo;
        this.records.push(record);
        this.records.sort((a, b) => a.date.localeCompare(b.date));
        this.save();
        console.log(chalk.green(`Запись добавлена: ${date} - ${weight} кг${photo ? ` (фото: ${photo})` : ''}`));
    }

    listRecords() {
        if (this.records.length === 0) {
            console.log(chalk.yellow('Нет записей.'));
            return;
        }
        console.log(chalk.cyan('Дата       | Вес (кг) | Фото'));
        for (const r of this.records) {
            const photo = r.photo || '-';
            console.log(`${r.date} | ${r.weight.toFixed(1)} | ${photo}`);
        }
    }

    showChart() {
        if (this.records.length === 0) {
            console.log(chalk.yellow('Нет данных для графика.'));
            return;
        }
        const weights = this.records.map(r => r.weight);
        const minW = Math.min(...weights);
        const maxW = Math.max(...weights);
        if (maxW === minW) {
            console.log('Все значения одинаковы.');
            return;
        }
        const scale = 20 / (maxW - minW);
        console.log(chalk.cyan('График изменения веса (кг):'));
        for (const r of this.records) {
            const barLen = Math.floor((r.weight - minW) * scale) + 1;
            const bar = '█'.repeat(barLen);
            const photo = r.photo || '';
            console.log(`${r.date} ${chalk.green(bar)} ${r.weight.toFixed(1)}  ${photo}`);
        }
    }

    showStats() {
        if (this.records.length === 0) {
            console.log(chalk.yellow('Нет данных.'));
            return;
        }
        const weights = this.records.map(r => r.weight);
        const minW = Math.min(...weights);
        const maxW = Math.max(...weights);
        const avgW = weights.reduce((a, b) => a + b, 0) / weights.length;
        const first = weights[0];
        const last = weights[weights.length - 1];
        const diff = last - first;
        const trend = diff > 0 ? '📈 растёт' : diff < 0 ? '📉 падает' : '➡️ стабилен';
        console.log(chalk.cyan('Статистика:'));
        console.log(`  Минимальный: ${chalk.green(minW.toFixed(1))} кг`);
        console.log(`  Максимальный: ${chalk.red(maxW.toFixed(1))} кг`);
        console.log(`  Средний: ${chalk.yellow(avgW.toFixed(1))} кг`);
        console.log(`  Тренд: ${chalk.magenta(trend)} (${diff.toFixed(1)} кг)`);
    }

    export(filename) {
        const ext = filename.split('.').pop().toLowerCase();
        let content;
        if (ext === 'json') {
            content = JSON.stringify(this.records, null, 2);
        } else if (ext === 'csv') {
            const header = 'date,weight,photo\n';
            const rows = this.records.map(r => `${r.date},${r.weight},${r.photo || ''}`).join('\n');
            content = header + rows;
        } else {
            console.log(chalk.red('Неподдерживаемый формат. Используйте .json или .csv'));
            return;
        }
        fs.writeFileSync(filename, content);
        console.log(chalk.green(`Экспортировано в ${filename}`));
    }
}

program
    .option('--add <weight>', 'Добавить вес (кг)', parseFloat)
    .option('--date <date>', 'Дата (YYYY-MM-DD)')
    .option('--photo <path>', 'Путь к фотографии')
    .option('--list', 'Показать записи')
    .option('--chart', 'Показать график')
    .option('--stats', 'Показать статистику')
    .option('--export <file>', 'Экспорт в файл')
    .parse(process.argv);

const opts = program.opts();
const tracker = new WeightTracker();

if (opts.add !== undefined) {
    tracker.addRecord(opts.add, opts.date, opts.photo);
} else if (opts.list) {
    tracker.listRecords();
} else if (opts.chart) {
    tracker.showChart();
} else if (opts.stats) {
    tracker.showStats();
} else if (opts.export) {
    tracker.export(opts.export);
} else {
    program.help();
}
