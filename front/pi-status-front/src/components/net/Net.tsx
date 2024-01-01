import React from "react";

import NetChart from "./NetChart";
import { formatBytes } from "../../utils";

import "./Net.css";

type NetProps = {
    netSpeeds: NetValues[];
    netMax: number;
    netTotals: NetValues;
};

export default function Net(props: NetProps) {
    return (
        <div className="stats-container">
            <div className="net-chart">
                {props.netSpeeds.length != 0 ? (
                    <div className="net-stats-container">
                        <div>
                            ↓{" "}
                            {formatBytes(
                                props.netSpeeds[props.netSpeeds.length - 1]
                                    .download,
                                { speed: true, space: true }
                            )}
                        </div>
                        <div>{formatBytes(props.netTotals.download, {})}</div>
                    </div>
                ) : (
                    ""
                )}
                <NetChart
                    netSpeeds={props.netSpeeds}
                    netMax={props.netMax}
                    dataKey="download"
                    color="#f28779"
                    chartClass="down-chart-container"
                />
            </div>
            <div className="net-chart">
                {props.netSpeeds.length != 0 ? (
                    <div className="net-stats-container">
                        <div>
                            ↑{" "}
                            {formatBytes(
                                props.netSpeeds[props.netSpeeds.length - 1]
                                    .upload,
                                { speed: true, space: true }
                            )}
                        </div>
                        <div>{formatBytes(props.netTotals.upload, {})}</div>
                    </div>
                ) : (
                    ""
                )}
                <NetChart
                    netSpeeds={props.netSpeeds}
                    netMax={props.netMax}
                    dataKey="upload"
                    color="#6ccdff"
                    chartClass="up-chart-container"
                />
            </div>
        </div>
    );
}
