import './Ram.css'
import { formatBytes } from "../../utils"

import { useState, useEffect } from 'react'

import {
    PieChart,
    Pie,
    Cell,
    ResponsiveContainer
} from "recharts";

export default function Ram({ramUsage}) {
    const [pieData, setPieData] = useState([])

    useEffect(() => {
        if(Object.keys(ramUsage).length != 0) {
            setPieData([
                { value: ramUsage.used },
                { value: ramUsage.available}
            ])
        }
    }, [ramUsage])
    
    return <div class="stats-container ram-container">
        <ResponsiveContainer>
            <PieChart>
                <Pie
                    data={pieData}
                    cx="50%"
                    cy="50%"
                    startAngle={180}
                    endAngle={0}
                    innerRadius="90%"
                    outerRadius="100%"
                    fill="#95e6cb"
                    paddingAngle={1}
                    animationDuration={100}
                    dataKey="value"
                    stroke="#0b0e14"
                >
                    <Cell key="cell-used" fill="#95e6cb" />
                    <Cell key="cell-used" fill="#565b66" />
                </Pie>
            </PieChart>
        </ResponsiveContainer>
    </div>
}
