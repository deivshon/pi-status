import { CoreData } from "@/models/cpu";
import { DiskData } from "@/models/disk";
import { NetValues } from "@/models/net";
import { ProcessData } from "@/models/proc";
import { RamData } from "@/models/ram";
import { statusDataSchema } from "@/models/ws";
import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap/dist/js/bootstrap.bundle.min.js";
import { useEffect, useState } from "react";
import Cpu from "../cpu/Cpu";
import Mem from "../mem/Mem";
import Net from "../net/Net";
import Proc from "../procs/Procs";
import "./App.css";
import { ErrorBox } from "./ErrorBox";
import { emptyCoreData, emptyNetValues, emptyRamData } from "./defaults";
import { computeHostData } from "./updates/host";
import {
    computeUpdatedNetStats,
    getMaxNetTotalsInterface,
} from "./updates/net";
import {
    SwitchDirection,
    getNewSelectedInterface,
} from "./updates/selectedInterface";

const maxNetDataPoints = 30;

export default function App() {
    const [hostname, setHostname] = useState("");
    const [uptime, setUptime] = useState("");
    const [temp, setTemp] = useState(0);
    const [netSpeeds, setNetSpeeds] = useState<Record<string, NetValues[]>>({});
    const [netTotals, setNetTotals] = useState<Record<string, NetValues>>({});
    const [netMaxes, setNetMaxes] = useState<Record<string, number>>({});
    const [selectedNetInterface, setSelectedNetInterface] = useState<
        string | null
    >(null);
    const [cpuUsage, setCpuUsage] = useState<CoreData[]>([]);
    const [ramData, setRamData] = useState<RamData>(emptyRamData);
    const [disks, setDisks] = useState<DiskData[]>([]);
    const [processes, setProcesses] = useState<ProcessData[]>([]);
    const [dataParsingError, setDataParsingError] = useState<string | null>(
        null,
    );

    const selectedNet = (() => {
        if (!selectedNetInterface) {
            return {
                speeds: [],
                max: 0,
                totals: emptyNetValues,
                allowBack: false,
                allowForward: false,
            };
        }

        const selectedNetInterfaceIndex = Object.keys(netTotals)
            .sort()
            .indexOf(selectedNetInterface);
        return {
            speeds: netSpeeds[selectedNetInterface] ?? [],
            max: netMaxes[selectedNetInterface] ?? 0,
            totals: netTotals[selectedNetInterface] ?? emptyNetValues,
            allowBack: selectedNetInterfaceIndex !== 0,
            allowForward:
                selectedNetInterfaceIndex !== Object.keys(netTotals).length - 1,
        };
    })();

    const handleNewData = async (event: MessageEvent) => {
        if (dataParsingError) {
            return;
        }

        let rawData;
        try {
            rawData = JSON.parse(event.data);
        } catch (error) {
            setDataParsingError(
                "Fatal error: WebSocket message is not valid JSON",
            );
            return;
        }

        const parseResult = statusDataSchema.safeParse(rawData);
        if (!parseResult.success) {
            setDataParsingError(
                `Fatal error, malformed message: ${parseResult.error}`,
            );
            return;
        }

        const newData = parseResult.data;

        if (newData.host) {
            const computedHostData = computeHostData(newData.host);
            setHostname(computedHostData.hostname);
            setUptime(computedHostData.uptime);
        }

        const rawNetData = newData.net_stats;
        if (rawNetData) {
            setNetSpeeds((prevSpeeds) => {
                const newNetStats = computeUpdatedNetStats(
                    rawNetData,
                    prevSpeeds,
                    maxNetDataPoints,
                );

                setNetTotals(newNetStats.netTotals);
                setNetMaxes(newNetStats.netMaxes);
                setSelectedNetInterface((prevSelected) =>
                    prevSelected && prevSelected in newNetStats.netTotals
                        ? prevSelected
                        : getMaxNetTotalsInterface(newNetStats.netTotals),
                );

                return newNetStats.netSpeeds;
            });
        }

        if (newData.temp) {
            setTemp(Math.round(newData.temp));
        }

        if (newData.cpu_usage) {
            setCpuUsage(newData.cpu_usage);
        }

        if (newData.ram) {
            setRamData(newData.ram);
        }

        if (newData.disk) {
            setDisks(newData.disk);
        }

        if (newData.proc) {
            setProcesses(newData.proc);
        }
    };

    useEffect(() => {
        const socket = new WebSocket(`ws://localhost:8080/ws_data`);

        socket.addEventListener("message", handleNewData);

        return () => socket.close();
    }, []);

    const switchInterface = (
        direction: SwitchDirection,
        netTotals: Record<string, NetValues>,
    ) => {
        const interfaceNames = Object.keys(netTotals).sort();
        setSelectedNetInterface((prev) => {
            if (!prev) {
                return getMaxNetTotalsInterface(netTotals);
            }

            return getNewSelectedInterface(
                prev,
                direction,
                netTotals,
                interfaceNames,
            );
        });
    };

    if (dataParsingError) {
        return <ErrorBox error={dataParsingError} />;
    }

    return (
        <div>
            <div className="host-bar">
                <p>{hostname}</p>
                <p>Up {uptime}</p>
            </div>
            <ul
                className="nav nav-pills justify-content-center flex flex-wrap"
                id="pills-tab"
                role="tablist"
            >
                <li className="nav-item cpu-pill" role="presentation">
                    <button
                        className="nav-link active"
                        id="pills-cpu-tab"
                        data-bs-toggle="pill"
                        data-bs-target="#pills-cpu"
                        type="button"
                        role="tab"
                    >
                        CPU
                    </button>
                </li>
                <li className="nav-item mem-pill" role="presentation">
                    <button
                        className="nav-link"
                        id="pills-mem-tab"
                        data-bs-toggle="pill"
                        data-bs-target="#pills-mem"
                        type="button"
                        role="tab"
                    >
                        MEM
                    </button>
                </li>
                <li className="nav-item net-pill" role="presentation">
                    <button
                        className="nav-link"
                        id="pills-net-tab"
                        data-bs-toggle="pill"
                        data-bs-target="#pills-net"
                        type="button"
                        role="tab"
                    >
                        NET
                    </button>
                </li>
                <li className="nav-item proc-pill" role="presentation">
                    <button
                        className="nav-link"
                        id="pills-proc-tab"
                        data-bs-toggle="pill"
                        data-bs-target="#pills-proc"
                        type="button"
                        role="tab"
                    >
                        PS
                    </button>
                </li>
            </ul>
            <div className="tab-content w-100" id="pills-tabContent">
                <div
                    className="tab-pane fade w-100 show active"
                    id="pills-cpu"
                    role="tabpanel"
                >
                    <Cpu temp={temp} cpuUsage={cpuUsage} />
                </div>
                <div
                    className="tab-pane fade w-100"
                    id="pills-mem"
                    role="tabpanel"
                >
                    <Mem ram={ramData} disks={disks} />
                </div>
                <div
                    className="tab-pane fade w-100"
                    id="pills-net"
                    role="tabpanel"
                    aria-labelledby="pills-net-tab"
                >
                    <div
                        id="interface-selector"
                        className="d-flex align-items-center justify-content-center"
                    >
                        <button
                            id="net-interface-prev"
                            onClick={() =>
                                switchInterface(SwitchDirection.Back, netTotals)
                            }
                        >
                            {selectedNet.allowBack && "ᐸ"}
                        </button>
                        <span className="flex-grow-1 text-center">
                            {selectedNetInterface}
                        </span>
                        <button
                            id="net-interface-next"
                            onClick={() =>
                                switchInterface(
                                    SwitchDirection.Forward,
                                    netTotals,
                                )
                            }
                        >
                            {selectedNet.allowForward && "ᐳ"}
                        </button>
                    </div>
                    <Net
                        netSpeeds={selectedNet.speeds}
                        netMax={selectedNet.max}
                        netTotals={selectedNet.totals}
                    />
                </div>
                <div
                    className="tab-pane fade w-100"
                    id="pills-proc"
                    role="tabpanel"
                >
                    <Proc
                        procs={processes}
                        mainCpuUsage={
                            cpuUsage.length > 0 ? cpuUsage[0] : emptyCoreData
                        }
                    />
                </div>
            </div>
        </div>
    );
}
