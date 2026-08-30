// WeightTracker.kt
import com.beust.jcommander.JCommander
import com.beust.jcommander.Parameter
import com.google.gson.GsonBuilder
import com.google.gson.reflect.TypeToken
import java.io.File
import java.time.LocalDate

data class Record(val date: String, val weight: Double, val photo: String? = null)

class WeightTracker {
    @Parameter(names = ["--add"])
    private var addWeight: Double? = null

    @Parameter(names = ["--date"])
    private var date: String? = null

    @Parameter(names = ["--photo"])
    private var photo: String? = null

    @Parameter(names = ["--list"])
    private var list: Boolean = false

    @Parameter(names = ["--chart"])
    private var chart: Boolean = false

    @Parameter(names = ["--stats"])
    private var stats: Boolean = false

    @Parameter(names = ["--export"])
    private var exportFile: String? = null

    private val dataFile = "weight.json"
    private val gson = GsonBuilder().setPrettyPrinting().create()
    private val type = object : TypeToken<MutableList<Record>>() {}.type
    private val records = mutableListOf<Record>()

    private fun load() {
        val f = File(dataFile)
        if (!f.exists()) return
        try {
            val json = f.readText()
            val list = gson.fromJson<MutableList<Record>>(json, type)
            records.addAll(list)
            records.sortBy { it.date }
        } catch (e: Exception) { /* ignore */ }
    }

    private fun save() {
        val json = gson.toJson(records)
        File(dataFile).writeText(json)
    }

    private fun addRecord(weight: Double, date: String?, photo: String?) {
        val d = date ?: LocalDate.now().toString()
        val rec = Record(d, weight, photo)
        records.add(rec)
        records.sortBy { it.date }
        save()
        print("\u001B[32mЗапись добавлена: $d - $weight кг")
        if (photo != null) print(" (фото: $photo)")
        println("\u001B[0m")
    }

    private fun listRecords() {
        if (records.isEmpty()) {
            println("\u001B[33mНет записей.\u001B[0m")
            return
        }
        println("\u001B[36mДата       | Вес (кг) | Фото\u001B[0m")
        for (r in records) {
            val photo = r.photo ?: "-"
            println("${r.date} | ${"%.1f".format(r.weight)} | $photo")
        }
    }

    private fun showChart() {
        if (records.isEmpty()) {
            println("\u001B[33mНет данных для графика.\u001B[0m")
            return
        }
        val weights = records.map { it.weight }
        val minW = weights.minOrNull() ?: 0.0
        val maxW = weights.maxOrNull() ?: 0.0
        if (maxW == minW) {
            println("Все значения одинаковы.")
            return
        }
        val scale = 20.0 / (maxW - minW)
        println("\u001B[36mГрафик изменения веса (кг):\u001B[0m")
        for (r in records) {
            val barLen = ((r.weight - minW) * scale).toInt() + 1
            val bar = "█".repeat(barLen)
            val photo = r.photo?.let { " $it" } ?: ""
            println("${r.date} \u001B[32m$bar\u001B[0m ${"%.1f".format(r.weight)}$photo")
        }
    }

    private fun showStats() {
        if (records.isEmpty()) {
            println("\u001B[33mНет данных.\u001B[0m")
            return
        }
        val weights = records.map { it.weight }
        val minW = weights.minOrNull() ?: 0.0
        val maxW = weights.maxOrNull() ?: 0.0
        val avgW = weights.average()
        val first = weights.first()
        val last = weights.last()
        val diff = last - first
        val trend = when {
            diff > 0 -> "📈 растёт"
            diff < 0 -> "📉 падает"
            else -> "➡️ стабилен"
        }
        println("\u001B[36mСтатистика:\u001B[0m")
        println("  Минимальный: \u001B[32m${"%.1f".format(minW)}\u001B[0m кг")
        println("  Максимальный: \u001B[31m${"%.1f".format(maxW)}\u001B[0m кг")
        println("  Средний: \u001B[33m${"%.1f".format(avgW)}\u001B[0m кг")
        println("  Тренд: \u001B[35m$trend\u001B[0m (${if (diff >= 0) "+" else ""}${"%.1f".format(diff)} кг)")
    }

    private fun export(filename: String) {
        val ext = filename.substringAfterLast('.', "")
        val content = when (ext.lowercase()) {
            "json" -> gson.toJson(records)
            "csv" -> {
                val sb = StringBuilder("date,weight,photo\n")
                records.forEach { sb.appendLine("${it.date},${it.weight},${it.photo ?: ""}") }
                sb.toString()
            }
            else -> {
                println("\u001B[31mНеподдерживаемый формат. Используйте .json или .csv\u001B[0m")
                return
            }
        }
        File(filename).writeText(content)
        println("\u001B[32mЭкспортировано в $filename\u001B[0m")
    }

    fun run() {
        load()
        when {
            addWeight != null -> addRecord(addWeight!!, date, photo)
            list -> listRecords()
            chart -> showChart()
            stats -> showStats()
            exportFile != null -> export(exportFile!!)
            else -> println("Используйте --help для справки.")
        }
    }
}

fun main(args: Array<String>) {
    val tracker = WeightTracker()
    JCommander.newBuilder().addObject(tracker).build().parse(*args)
    tracker.run()
}
