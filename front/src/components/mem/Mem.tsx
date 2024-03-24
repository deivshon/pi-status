import { formatBytes } from "@/lib/bytes";
import { DiskData } from "@/models/disk";
import { RamData } from "@/models/ram";
import Ram from "./Ram";

import "./Mem.css";

type MemProps = {
    ram: RamData;
    disks: DiskData[];
};

export default function Mem(props: MemProps) {
    return (
        <div className="stats-container flex-column align-items-center w-100">
            <p>RAM {formatBytes(props.ram.total)}</p>
            <div className="w-100 d-flex flex-column align-items-center">
                <Ram
                    total={props.ram.total}
                    value={props.ram.used}
                    label="Used"
                />
            </div>
            <div className="w-100 d-flex flex-column align-items-center">
                <Ram
                    total={props.ram.total}
                    value={props.ram.available}
                    label="Available"
                />
            </div>
            <div className="w-100 d-flex flex-column align-items-center">
                <Ram
                    total={props.ram.total}
                    value={props.ram.free}
                    label="Free"
                />
            </div>
            <div className="w-100 d-flex flex-column align-items-center">
                <Ram
                    total={props.ram.total}
                    value={props.ram.cached}
                    label="Cached"
                />
            </div>
            <span>Disks</span>
            <table className="disks-container">
                <tr>
                    <th>Filesystem</th>
                    <th>Size</th>
                    <th>Avail</th>
                    <th>Use%</th>
                    <th>Mounted on</th>
                </tr>
                {props.disks.map((d, i) => (
                    <tr key={i}>
                        <td>{d.filesystem}</td>
                        <td>
                            {formatBytes(d.total, {
                                short: true,
                                space: false,
                                roundingDigits: 0,
                            })}
                        </td>
                        <td>
                            {formatBytes(d.available, {
                                short: true,
                                space: false,
                                roundingDigits: 0,
                            })}
                        </td>
                        <td>
                            {(
                                ((d.total - d.available) / d.total) *
                                100
                            ).toFixed(0)}
                            %
                        </td>
                        <td>{d.mountpoint}</td>
                    </tr>
                ))}
            </table>
        </div>
    );
}
