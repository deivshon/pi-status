import { formatBytes } from "../../utils"

import './Net.css'

import {
    AreaChart,
    Area,
    YAxis,
    ResponsiveContainer
} from "recharts";

import NetChart from './NetChart'

export default function Net({netSpeeds, netMax, netTotals}) {
    return <div class="stats-container">
        <div className="net-chart">
            {netSpeeds.length != 0 ? `↓ ${formatBytes(netSpeeds[netSpeeds.length - 1].download, true)} | ${formatBytes(netTotals.download)}` : ""}
            <NetChart
                netSpeeds={netSpeeds}
                netMax={netMax}
                dataKey="download"
                color="#f28779"
                chartClass="down-chart-container"
            />
        </div>
        <div className="net-chart">
            {netSpeeds.length != 0 ? `↑ ${formatBytes(netSpeeds[netSpeeds.length - 1].upload, true)} | ${formatBytes(netTotals.upload)}` : ""}
            <NetChart
                netSpeeds={netSpeeds}
                netMax={netMax}
                dataKey="upload"
                color="#6ccdff"
                chartClass="up-chart-container"
            />
        </div>
    </div>
}
