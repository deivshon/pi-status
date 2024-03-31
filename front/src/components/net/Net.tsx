import { formatBytes } from "@/lib/bytes";
import { twColors } from "@/lib/consts";
import { NetTransferType, NetValues } from "@/models/net";
import NetChart from "./NetChart";

type NetProps = {
    netSpeeds: NetValues[];
    netMax: number;
    netTotals: NetValues;
};

export default function Net(props: NetProps) {
    return (
        <div className="flex flex-col gap-2 md:flex-row md:gap-4">
            <div className="mb-4 flex h-[25vh] w-[96vw] flex-col gap-1 md:h-[72vh] md:w-[50vw]">
                {props.netSpeeds.length !== 0 && (
                    <div className="flex justify-between">
                        <p>
                            ▼{" "}
                            {formatBytes(
                                props.netSpeeds[props.netSpeeds.length - 1]
                                    .download,
                                { speed: true, space: true },
                            )}
                        </p>
                        <p>{formatBytes(props.netTotals.download, {})}</p>
                    </div>
                )}
                <NetChart
                    netSpeeds={props.netSpeeds}
                    netMax={props.netMax}
                    dataKey={NetTransferType.Download}
                    color={twColors["ayu-red"]}
                    className="border-2 border-ayu-red"
                />
            </div>
            <div className="mb-4 flex h-[25vh] w-[96vw] flex-col gap-1 md:h-[72vh] md:w-[50vw]">
                {props.netSpeeds.length !== 0 && (
                    <div className="flex justify-between">
                        <p>
                            ▲{" "}
                            {formatBytes(
                                props.netSpeeds[props.netSpeeds.length - 1]
                                    .upload,
                                { speed: true, space: true },
                            )}
                        </p>
                        <p>{formatBytes(props.netTotals.upload, {})}</p>
                    </div>
                )}
                <NetChart
                    netSpeeds={props.netSpeeds}
                    netMax={props.netMax}
                    dataKey={NetTransferType.Upload}
                    color={twColors["ayu-cyan"]}
                    className="border-2 border-ayu-cyan"
                />
            </div>
        </div>
    );
}
