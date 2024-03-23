import { NetTransferType, NetValues } from "@/models/net";
import { Area, AreaChart, ResponsiveContainer, YAxis } from "recharts";
import "./Net.css";

type NetChartProps = {
    netSpeeds: NetValues[];
    netMax: number;
    dataKey: NetTransferType;
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
                    type="linear"
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
