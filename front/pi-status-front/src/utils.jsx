const shortBytePrefixes = [
    "B",
    "K",
    "M",
    "G",
    "T",
    "P"
]

const bytePrefixes = [
    "B",
    "KiB",
    "MiB",
    "GiB",
    "TiB",
    "PiB"
]

export const formatBytes = (bytes, {
    speed = false,
    short = false,
    roundTreshold = 1025,
    absoluteRoundTreshold = 0
}) => {
    let roundAbsolute = bytes > absoluteRoundTreshold

    let i = 0

    while (bytes > 1024 && i < bytePrefixes.length - 1) {
        bytes /= 1024
        i++
    }

    return `${bytes.toFixed(i != 0 && bytes < roundTreshold && roundAbsolute ? 2 : 0)} ${short ? shortBytePrefixes[i] : bytePrefixes[i]}${speed ? "/s" : ""}`
}
