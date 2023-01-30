import './Mem.css'
import { formatBytes } from "../../utils"

import RamStat from './RamStat'

export default function Mem({ramUsage, disks}) {
    return <div class="stats-container flex-column align-items-center w-100">
            <div>
                RAM {ramUsage.total ? formatBytes(ramUsage.total, {}) : 0}
            </div>
            <div></div>
            <div className="w-100 d-flex flex-column align-items-center">
                <RamStat
                    total={ramUsage.total || 0}
                    stat={ramUsage.used ? ramUsage.used : 0}
                    label="Used"
                />
            </div>
            <div className="w-100 d-flex flex-column align-items-center">
                <RamStat
                    total={ramUsage.total || 0}
                    stat={ramUsage.available ? ramUsage.available : 0}
                    label="Available"
                />
            </div>
            <div className="w-100 d-flex flex-column align-items-center">
                <RamStat
                    total={ramUsage.total || 0}
                    stat={ramUsage.free ? ramUsage.free : 0}
                    label="Free"
                />
            </div>
            <div className="w-100 d-flex flex-column align-items-center">
                <RamStat
                    total={ramUsage.total || 0}
                    stat={ramUsage.cached ? ramUsage.cached : 0}
                    label="Cached"
                />
            </div>
            Disks
            <table class="disks-container">
                <tr>
                    <th>Filesystem</th>
                    <th>Size</th>
                    <th>Avail</th>
                    <th>Use%</th>
                    <th>Mounted on</th>
                </tr>
                    {disks.map(d => <tr>
                        <td>{d.filesystem}</td>
                        <td>{formatBytes(d.total, {short: true, space: false, roundingDigits: 0})}</td>
                        <td>{formatBytes(d.available, {short: true, space: false, roundingDigits: 0})}</td>
                        <td>{(((d.total - d.available) / d.total) * 100).toFixed(0)}%</td>
                        <td>{d.mountpoint}</td>
                    </tr>)}
            </table>
    </div>
}
