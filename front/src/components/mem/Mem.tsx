import { FormatBytesOpts, formatBytes } from "@/lib/bytes";
import { DiskData } from "@/models/disk";
import { RamData } from "@/models/ram";
import RamBar from "./Ram";

type MemProps = {
    ram: RamData;
    disks: DiskData[];
};

const diskSpaceFormat: FormatBytesOpts = {
    short: true,
    space: false,
    roundingDigits: 0,
};

export default function Mem(props: MemProps) {
    const disks = props.disks.map((d) => ({
        filesystem: d.filesystem,
        total: formatBytes(d.total, diskSpaceFormat),
        available: formatBytes(d.available, diskSpaceFormat),
        use: (((d.total - d.available) / d.total) * 100).toFixed(0),
        mountpoint: d.mountpoint,
    }));

    return (
        <div className="flex w-full flex-col gap-2">
            <p className="mb-1">RAM {formatBytes(props.ram.total)}</p>
            <div className="flex w-full flex-col items-center">
                <RamBar
                    total={props.ram.total}
                    value={props.ram.used}
                    label="Used"
                />
            </div>
            <div className="flex w-full flex-col items-center">
                <RamBar
                    total={props.ram.total}
                    value={props.ram.available}
                    label="Available"
                />
            </div>
            <div className="flex w-full flex-col items-center">
                <RamBar
                    total={props.ram.total}
                    value={props.ram.free}
                    label="Free"
                />
            </div>
            <div className="flex w-full flex-col items-center">
                <RamBar
                    total={props.ram.total}
                    value={props.ram.cached}
                    label="Cached"
                />
            </div>
            <span className="mb-1 mt-2 md:mb-4">Disks</span>
            <table className="border-ayu-green border-[1px] text-left text-[0.5rem] md:text-base">
                <thead>
                    <tr>
                        <th className="border-ayu-green border-[1px] px-[0.15rem] md:px-1">
                            Filesystem
                        </th>
                        <th className="border-ayu-green border-[1px] px-[0.15rem] md:px-1">
                            Size
                        </th>
                        <th className="border-ayu-green border-[1px] px-[0.15rem] md:px-1">
                            Avail
                        </th>
                        <th className="border-ayu-green border-[1px] px-[0.15rem] md:px-1">
                            Use%
                        </th>
                        <th className="border-ayu-green border-[1px] px-[0.15rem] md:px-1">
                            Mounted on
                        </th>
                    </tr>
                </thead>
                <tbody>
                    {disks.map((disk, idx) => (
                        <tr key={idx}>
                            <td className="border-ayu-green border-[1px] px-[0.15rem] md:px-1">
                                {disk.filesystem}
                            </td>
                            <td className="border-ayu-green border-[1px] px-[0.15rem] md:px-1">
                                {disk.total}
                            </td>
                            <td className="border-ayu-green border-[1px] px-[0.15rem] md:px-1">
                                {disk.available}
                            </td>
                            <td className="border-ayu-green border-[1px] px-[0.15rem] md:px-1">
                                {disk.use}%
                            </td>
                            <td className="border-ayu-green border-[1px] px-[0.15rem] md:px-1">
                                {disk.mountpoint}
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    );
}
