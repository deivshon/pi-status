import { formatBytes } from "../../utils"

import './Net.css'

import {
    AreaChart,
    Area,
    YAxis,
    ResponsiveContainer
} from "recharts";

export default function Net({netSpeeds, netMax, netTotals}) {
    return <div class="stats-container">
        <div className="net-chart">
            {netSpeeds.length != 0 ? `↓ ${formatBytes(netSpeeds[netSpeeds.length - 1].download, true)} | ${formatBytes(netTotals.download)}` : ""}
            <ResponsiveContainer width="100%" height="100%">
                <AreaChart
                    data={netSpeeds}
                    style={{ border: "2px solid #f28779" }}
                >
                <YAxis domain={[0, netMax]} hide={true} allowDataOverflow={true}/>
                <Area
                    type="monotone"
                    dataKey="download"
                    stroke="#f28779"
                    fill="#f28779"
                    animationDuration={100}
                    dot={false}
                />
                </AreaChart>
            </ResponsiveContainer>
        </div>
        <div className="net-chart">
            {netSpeeds.length != 0 ? `↑ ${formatBytes(netSpeeds[netSpeeds.length - 1].upload, true)} | ${formatBytes(netTotals.upload)}` : ""}
            <ResponsiveContainer width="100%" height="100%">
                <AreaChart
                    data={netSpeeds}
                    style={{ border: "2px solid #6ccdff" }}
                >
                <YAxis domain={[0, netMax]} hide={true} allowDataOverflow={true}/>
                <Area
                    type="monotone"
                    dataKey="upload"
                    stroke="#6ccdff"
                    fill="#6ccdff"
                    animationDuration={100}
                    dot={false}
                />
                </AreaChart>
            </ResponsiveContainer>
        </div>
    </div>
}
