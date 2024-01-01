import React from "react";
import "./Net.css";

import { AreaChart, Area, YAxis, ResponsiveContainer } from "recharts";

type NetChartProps = {
    netSpeeds: NetValues[];
    netMax: number;
    dataKey: string;
    color: string;
    chartClass: string;
};

export default function NetChart(props: NetChartProps) {
    return (
        <ResponsiveContainer
            width="100%"
            height="100%"
            className={props.chartClass}
        >
            <AreaChart data={props.netSpeeds}>
                <YAxis
                    domain={[0, props.netMax]}
                    hide={true}
                    allowDataOverflow={true}
                />

                <Area
                    type="monotone"
                    dataKey={props.dataKey}
                    stroke={props.color}
                    fill={props.color}
                    animationDuration={100}
                    dot={false}
                />
            </AreaChart>
        </ResponsiveContainer>
    );
}
