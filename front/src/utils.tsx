const shortBytePrefixes = ["B", "K", "M", "G", "T", "P"];
const bytePrefixes = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];

export const formatBytes = (
    bytes: number,
    {
        speed = false,
        short = true,
        roundTreshold = 1025,
        absoluteRoundTreshold = 0,
        roundingDigits = 2,
        space = false,
    },
) => {
    const roundAbsolute = bytes > absoluteRoundTreshold;

    let i = 0;

    while (bytes > 1024 && i < bytePrefixes.length - 1) {
        bytes /= 1024;
        i++;
    }

    return `${bytes.toFixed(
        i != 0 && bytes < roundTreshold && roundAbsolute ? roundingDigits : 0,
    )}${space ? " " : ""}${short ? shortBytePrefixes[i] : bytePrefixes[i]}${
        speed ? "/s" : ""
    }`;
};
