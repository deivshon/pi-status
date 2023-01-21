const formatBytes = (bytes, isSpeed = false) => {
    const prefixes = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"]

    let i = 0
    while(bytes > 1024 && i < prefixes.length - 1) {
        bytes /= 1024
        i++
    }

    return `${bytes.toFixed(2)} ${prefixes[i]}${isSpeed ? "/s" : ""}`
}

const updateData = async () => {
    let data = await (await fetch("/data")).json()

    $("#temp").html(data.temp + "°C")
    $("#up-speed").html(formatBytes(data.net_stats.upload_speed, true))
    $("#down-speed").html(formatBytes(data.net_stats.download_speed, true))
    $("#up-tot").html(formatBytes(data.net_stats.upload_total))
    $("#down-tot").html(formatBytes(data.net_stats.download_total))

}

updateData()
setInterval(updateData, 1000)
