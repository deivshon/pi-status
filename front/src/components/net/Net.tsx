import { formatBytes } from "@/lib/bytes";
import { NetTransferType, NetValues } from "@/models/net";
import "./Net.css";
import NetChart from "./NetChart";

type NetProps = {
    netSpeeds: NetValues[];
    netMax: number;
    netTotals: NetValues;
};

export default function Net(props: NetProps) {
    return (
        <div className="stats-container">
            <div className="net-chart">
                {props.netSpeeds.length !== 0 ? (
                    <div className="net-stats-container">
                        <div>
                            ▼{" "}
                            {formatBytes(
                                props.netSpeeds[props.netSpeeds.length - 1]
                                    .download,
                                { speed: true, space: true },
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
                    dataKey={NetTransferType.Download}
                    color="#f28779"
                    chartClass="down-chart-container"
                />
            </div>
            <div className="net-chart">
                {props.netSpeeds.length !== 0 ? (
                    <div className="net-stats-container">
                        <div>
                            ▲{" "}
                            {formatBytes(
                                props.netSpeeds[props.netSpeeds.length - 1]
                                    .upload,
                                { speed: true, space: true },
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
                    dataKey={NetTransferType.Upload}
                    color="#6ccdff"
                    chartClass="up-chart-container"
                />
            </div>
        </div>
    );
}
