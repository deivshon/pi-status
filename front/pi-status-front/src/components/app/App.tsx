import { useState, useEffect } from "react";

import { StatusData } from "./models";
import Cpu from "../cpu/Cpu";
import Net from "../net/Net";
import Mem from "../mem/Mem";
import Proc from "../procs/Procs";

import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap/dist/js/bootstrap.bundle.min.js";
import "./App.css";

import { CoreData } from "../cpu/models";
import { NetValues } from "../net/models";
import { DiskData, RamData } from "../mem/models";

export default function App() {
    const [runOnce, setRunOnce] = useState(false);

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

    const handleNewData = async (event: MessageEvent) => {
        const newData = JSON.parse(event.data) as StatusData;

        if (newData.host) {
            setHostname(newData.host.hostname);

            let uptime = newData.host.uptime;
            if (uptime) {
                if (uptime < 3600) {
                    setUptime(`${(uptime / 60).toFixed(0)} min`);
                } else {
                    setUptime(`${(uptime / 3600).toFixed(0)} hours`);
                }
            }
        }

        if (newData.net_stats) {
            setNetSpeeds((prev) => {
                let newNetSpeeds: Record<string, NetValues[]> = {};
                for (const interfaceData of newData.net_stats!) {
                    let interfaceName = interfaceData.interface;
                    let interfaceSpeeds = {
                        download: interfaceData.download_speed,
                        upload: interfaceData.upload_speed,
                    };

                    if (!(interfaceName in prev)) {
                        newNetSpeeds[interfaceName] = [interfaceSpeeds];
                    } else {
                        newNetSpeeds[interfaceName] = [
                            ...prev[interfaceName],
                            interfaceSpeeds,
                        ];
                    }

                    if (newNetSpeeds[interfaceName].length > 30) {
                        newNetSpeeds[interfaceName] =
                            newNetSpeeds[interfaceName].slice(1);
                    }
                }

                return newNetSpeeds;
            });

            let newNetTotals: Record<string, NetValues> = {};

            let maxTotal: number | null = null;
            let maxTotalInterfaceName: string | null = null;
            for (const interfaceData of newData.net_stats!) {
                newNetTotals[interfaceData.interface] = {
                    download: interfaceData.download_total,
                    upload: interfaceData.upload_total,
                };

                let netTotalSum =
                    interfaceData.download_total + interfaceData.upload_total;
                if (maxTotal === null || netTotalSum > maxTotal) {
                    maxTotal = netTotalSum;
                    maxTotalInterfaceName = interfaceData.interface;
                }
            }

            setNetTotals(newNetTotals);
            setSelectedNetInterface((prev) =>
                prev && prev in newNetTotals ? prev : maxTotalInterfaceName
            );
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
        let newNetMaxes: Record<string, number> = {};
        for (const interfaceName in netSpeeds) {
            let interfaceSpeeds = netSpeeds[interfaceName];

            newNetMaxes[interfaceName] = Math.max(
                ...interfaceSpeeds.map((v) => v.download),
                ...interfaceSpeeds.map((v) => v.upload)
            );
        }

        setNetMaxes(newNetMaxes);
    }, [netSpeeds]);

    useEffect(() => {
        if (!runOnce) {
            const socket = new WebSocket(
                `ws://${window.location.host}/ws_data`
            );

            socket.addEventListener("message", handleNewData);
            setRunOnce(true);
        }
    }, [runOnce]);

    return (
        <div>
            <div className="host-bar">
                <div>{hostname}</div>
                <div>Up {uptime}</div>
            </div>
            <ul
                className="nav nav-pills mb-3 flex flex-wrap justify-content-center"
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
