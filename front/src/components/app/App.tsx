import { CoreData } from "@/models/cpu";
import { DiskData } from "@/models/disk";
import { NetValues } from "@/models/net";
import { ProcessData } from "@/models/proc";
import { RamData } from "@/models/ram";
import { StatusDataSchema } from "@/models/ws";
import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap/dist/js/bootstrap.bundle.min.js";
import { useEffect, useState } from "react";
import Cpu from "../cpu/Cpu";
import Mem from "../mem/Mem";
import Net from "../net/Net";
import Proc from "../procs/Procs";
import "./App.css";
import { ErrorBox } from "./ErrorBox";
import { computeHostData } from "./data/host";
import { computeUpdatedNetStats } from "./data/net";

enum SwitchDirection {
    Back = 0,
    Forward = 1,
}

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
    const [ramData, setRamData] = useState<RamData>({
        available: 0,
        cached: 0,
        free: 0,
        total: 0,
        used: 0,
    });
    const [disks, setDisks] = useState<DiskData[]>([]);
    const [processes, setProcesses] = useState<ProcessData[]>([]);
    const [dataParsingError, setDataParsingError] = useState<string | null>(
        null,
    );

    const getMaxNetTotalsInterface = (
        totals: Record<string, NetValues>,
    ): string | null => {
        let maxInterfaceName: string | null = null;
        let maxInterfaceSum: number | null = null;
        for (const interfaceName of Object.keys(totals)) {
            const interfaceTotals = totals[interfaceName];
            const interfaceSum =
                interfaceTotals.download + interfaceTotals.upload;

            if (maxInterfaceSum === null || interfaceSum > maxInterfaceSum) {
                maxInterfaceSum = interfaceSum;
                maxInterfaceName = interfaceName;
            }
        }

        return maxInterfaceName;
    };

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

        const parseResult = StatusDataSchema.safeParse(rawData);
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
        const socket = new WebSocket(`ws://${window.location.host}/ws_data`);

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

            const prevIndex = interfaceNames.indexOf(prev);
            let newSelectedInterface: string | null = null;
            if (prevIndex === -1) {
                newSelectedInterface = getMaxNetTotalsInterface(netTotals);
            } else if (
                (prevIndex === 0 && direction === SwitchDirection.Back) ||
                (prevIndex === interfaceNames.length - 1 &&
                    direction === SwitchDirection.Forward)
            ) {
                newSelectedInterface = prev;
            } else if (direction === SwitchDirection.Back) {
                newSelectedInterface = interfaceNames[prevIndex - 1] ?? null;
            } else if (direction === SwitchDirection.Forward) {
                newSelectedInterface = interfaceNames[prevIndex + 1] ?? null;
            }

            return newSelectedInterface !== null &&
                interfaceNames.includes(newSelectedInterface)
                ? newSelectedInterface
                : prev;
        });
    };

    if (dataParsingError) {
        return <ErrorBox error={dataParsingError} />;
    }

    return (
        <div>
            <div className="host-bar">
                <div>{hostname}</div>
                <div>Up {uptime}</div>
            </div>
            <ul
                className="nav nav-pills flex flex-wrap justify-content-center"
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
                    <p
                        id="interface-selector"
                        className="d-flex align-items-center justify-content-center"
                    >
                        <button
                            id="net-interface-prev"
                            onClick={() =>
                                switchInterface(SwitchDirection.Back, netTotals)
                            }
                        >
                            {selectedNetInterface !== null &&
                            Object.keys(netTotals)
                                .sort()
                                .indexOf(selectedNetInterface) !== 0
                                ? "ᐸ"
                                : ""}
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
                            {selectedNetInterface !== null &&
                            Object.keys(netTotals)
                                .sort()
                                .indexOf(selectedNetInterface) !==
                                Object.keys(netTotals).length - 1
                                ? "ᐳ"
                                : ""}
                        </button>
                    </p>
                    <Net
                        netSpeeds={
                            selectedNetInterface &&
                            netSpeeds[selectedNetInterface]
                                ? netSpeeds[selectedNetInterface]
                                : []
                        }
                        netMax={
                            selectedNetInterface &&
                            netMaxes[selectedNetInterface]
                                ? netMaxes[selectedNetInterface]
                                : 0
                        }
                        netTotals={
                            selectedNetInterface &&
                            netTotals[selectedNetInterface]
                                ? netTotals[selectedNetInterface]
                                : {
                                      download: 0,
                                      upload: 0,
                                  }
                        }
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
                            cpuUsage.length > 0
                                ? cpuUsage[0]
                                : {
                                      user: 0,
                                      nice: 0,
                                      system: 0,
                                      idle: 0,
                                      iowait: 0,
                                      irq: 0,
                                      softirq: 0,
                                      steal: 0,
                                      guest: 0,
                                      guest_nice: 0,
                                  }
                        }
                    />
                </div>
            </div>
        </div>
    );
}
