import "./Mem.css";
import { formatBytes } from "../../utils";

import Ram from "./Ram";
import { DiskData, RamData } from "./models";

type MemProps = {
    ram: RamData;
    disks: DiskData[];
};

export default function Mem(props: MemProps) {
    return (
        <div className="stats-container flex-column align-items-center w-100">
            <div>
                RAM {props.ram.total ? formatBytes(props.ram.total, {}) : 0}
            </div>
            <div className="w-100 d-flex flex-column align-items-center">
                <Ram
                    total={props.ram.total || 0}
                    value={props.ram.used ? props.ram.used : 0}
                    label="Used"
                />
            </div>
            <div className="w-100 d-flex flex-column align-items-center">
                <Ram
                    total={props.ram.total || 0}
                    value={props.ram.available ? props.ram.available : 0}
                    label="Available"
                />
            </div>
            <div className="w-100 d-flex flex-column align-items-center">
                <Ram
                    total={props.ram.total || 0}
                    value={props.ram.free ? props.ram.free : 0}
                    label="Free"
                />
            </div>
            <div className="w-100 d-flex flex-column align-items-center">
                <Ram
                    total={props.ram.total || 0}
                    value={props.ram.cached ? props.ram.cached : 0}
                    label="Cached"
                />
            </div>
            Disks
            <table className="disks-container">
                <tr>
                    <th>Filesystem</th>
                    <th>Size</th>
                    <th>Avail</th>
                    <th>Use%</th>
                    <th>Mounted on</th>
                </tr>
                {props.disks.map((d) => (
                    <tr>
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
