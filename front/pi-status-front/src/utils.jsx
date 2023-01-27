export const formatBytes = (bytes, isSpeed = false) => {
    const prefixes = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"]

    let i = 0
    while (bytes > 1024 && i < prefixes.length - 1) {
        bytes /= 1024
        i++
    }

    return `${bytes.toFixed(i != 0 ? 2 : 0)} ${prefixes[i]}${isSpeed ? "/s" : ""}`
}
