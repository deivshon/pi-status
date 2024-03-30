import { formatBytes } from "@/lib/bytes";
import { DiskData } from "@/models/disk";
import { RamData } from "@/models/ram";
import RamBar from "./Ram";

type MemProps = {
    ram: RamData;
    disks: DiskData[];
};

export default function Mem(props: MemProps) {
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
                    {props.disks.map((d, i) => (
                        <tr key={i}>
                            <td className="border-ayu-green border-[1px] px-[0.15rem] md:px-1">
                                {d.filesystem}
                            </td>
                            <td className="border-ayu-green border-[1px] px-[0.15rem] md:px-1">
                                {formatBytes(d.total, {
                                    short: true,
                                    space: false,
                                    roundingDigits: 0,
                                })}
                            </td>
                            <td className="border-ayu-green border-[1px] px-[0.15rem] md:px-1">
                                {formatBytes(d.available, {
                                    short: true,
                                    space: false,
                                    roundingDigits: 0,
                                })}
                            </td>
                            <td className="border-ayu-green border-[1px] px-[0.15rem] md:px-1">
                                {(
                                    ((d.total - d.available) / d.total) *
                                    100
                                ).toFixed(0)}
                                %
                            </td>
                            <td className="border-ayu-green border-[1px] px-[0.15rem] md:px-1">
                                {d.mountpoint}
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    );
}
