import './Net.css'

import {
    AreaChart,
    Area,
    YAxis,
    ResponsiveContainer
} from "recharts";

export default function Net({netSpeeds, netMax, dataKey, color, chartClass}) {
    return <ResponsiveContainer
        width="100%"
        height="100%"
        className={chartClass}
    >
        <AreaChart
            data={netSpeeds}
        >

        <YAxis
            domain={[0, netMax]}
            hide={true}
            allowDataOverflow={true}
        />

        <Area
            type="monotone"
            dataKey={dataKey}
            stroke={color}
            fill={color}
            animationDuration={100}
            dot={false}
        />
        </AreaChart>
    </ResponsiveContainer>
}