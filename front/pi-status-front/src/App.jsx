import React from "react";

import { useState } from 'react'
import './App.css'

import {
    LineChart,
    Line,
    XAxis,
    YAxis,
    CartesianGrid,
    Tooltip,
    Legend,
    ResponsiveContainer
} from "recharts";
import { useEffect } from "react";

const formatBytes = (bytes, isSpeed = false) => {
    const prefixes = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"]

    let i = 0
    while(bytes > 1024 && i < prefixes.length - 1) {
        bytes /= 1024
        i++
    }

    return `${bytes.toFixed(2)} ${prefixes[i]}${isSpeed ? "/s" : ""}`
}

export default function App() {
    const [net, setNet] = useState([])
    const [temp, setTemp] = useState(0)

    const changeData = async () => {
        let newData = await (await fetch("/data")).json()
        if(net.length > 10) net.shift()

        setNet([
            ...net,
            {
                upload: newData.net_stats.upload_speed,
                download: newData.net_stats.download_speed

            }
        ])

        setTemp(newData.temp)
    }

    useEffect(() => {
        const interval = setInterval(changeData, 1000)
        return () => clearInterval(interval)
    })

    return (
        <div className="main-container">
            <div className="temp-chart">
                {net.length != 0 ? `↓ ${formatBytes(net[net.length - 1].download, true)} | ↑ ${formatBytes(net[net.length - 1].upload, true)}` : ""}
                <ResponsiveContainer width="100%" height="100%">
                    <LineChart
                        data={net}
                        style={{border: "2px solid black"}}
                    >
                        <Legend />
                        <Line
                            type="monotone"
                            dataKey="download"
                            stroke="#f9cf9a"
                            isAnimationActive={false}
                            activeDot={{ r: 8 }}
                        />
                        <Line
                            type="monotone"
                            dataKey="upload"
                            stroke="#82ca9d"
                            isAnimationActive={false}
                            activeDot={{ r: 8 }}
                        />
                    </LineChart>
                </ResponsiveContainer>
            </div>
            <div className="temp">
                Temperature: {temp} °C
            </div>
        </div>
    );
}
