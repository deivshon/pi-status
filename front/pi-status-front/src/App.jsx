import React from "react";

import { useState } from 'react'
import './App.css'

import {
    AreaChart,
    Area,
    YAxis,
    Legend,
    ResponsiveContainer
} from "recharts";
import { useEffect } from "react";

const formatBytes = (bytes, isSpeed = false) => {
    const prefixes = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"]

    let i = 0
    while (bytes > 1024 && i < prefixes.length - 1) {
        bytes /= 1024
        i++
    }

    return `${bytes.toFixed(2)} ${prefixes[i]}${isSpeed ? "/s" : ""}`
}

export default function App() {
    const [netSpeeds, setNetSpeeds] = useState([])
    const [netTotals, setNetTotals] = useState({})
    const [netMax, setNetMax] = useState(0)
    const [temp, setTemp] = useState(0)

    const changeData = async () => {
        let newData = await (await fetch("/data")).json()
        if (netSpeeds.length > 30) netSpeeds.shift()

        setNetSpeeds([
            ...netSpeeds,
            {
                download: newData.net_stats.download_speed,
                upload: newData.net_stats.upload_speed
            }
        ])

        setNetMax(Math.max(
            ...(netSpeeds.map(v => v.download)),
            ...(netSpeeds.map(v => v.upload)))
        )

        setNetTotals({
            download: newData.net_stats.download_total,
            upload: newData.net_stats.upload_total,
        })
        
        setTemp(Math.round(newData.temp))
    }

    useEffect(() => {
        const interval = setInterval(changeData, 1000)
        return () => clearInterval(interval)
    })

    return (
        <div>
            <ul class="nav nav-pills mb-3 flex flex-wrap justify-content-center" id="pills-tab" role="tablist">
                <li class="nav-item fs-6 net-pill" role="presentation">
                    <button class="nav-link active" id="pills-net-tab" data-bs-toggle="pill" data-bs-target="#pills-net" type="button" role="tab">Net</button>
                </li>
                <li class="nav-item fs-6 cpu-pill" role="presentation">
                    <button class="nav-link" id="pills-cpu-tab" data-bs-toggle="pill" data-bs-target="#pills-cpu" type="button" role="tab">CPU</button>
                </li>
            </ul>
            <div class="tab-content w-100" id="pills-tabContent">
                <div class="tab-pane fade show active w-100" id="pills-net" role="tabpanel" aria-labelledby="pills-net-tab">
                    <div class="stats-container">
                    <div className="temp-chart">
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
                                isAnimationActive={false}
                                dot={false}
                            />
                            </AreaChart>
                        </ResponsiveContainer>
                    </div>
                    <div className="temp-chart">
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
                                isAnimationActive={false}
                                dot={false}
                            />
                            </AreaChart>
                        </ResponsiveContainer>
                    </div>
                    </div>
                </div>
                <div class="tab-pane fade w-100" id="pills-cpu" role="tabpanel">
                    <div class="stats-container">
                    <div className="temp">
                        Temperature: {temp} °C
                    </div>
                    </div>
                </div>
            </div>
        </div>
    );
}
