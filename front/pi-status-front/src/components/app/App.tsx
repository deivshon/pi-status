import React from "react";
import { useState, useEffect } from "react";

import Cpu from "../cpu/Cpu";
import Net from "../net/Net";
import Mem from "../mem/Mem";
import Proc from "../procs/Procs";

import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap/dist/js/bootstrap.bundle.min.js";
import "./App.css";

import { CoreData } from "../cpu/models";

export default function App() {
    const [runOnce, setRunOnce] = useState(false);
    const [hostname, setHostname] = useState("");
    const [uptime, setUptime] = useState("");

    const [temp, setTemp] = useState(0);

    const [netSpeeds, setNetSpeeds] = useState<NetValues[]>([]);
    const [netTotals, setNetTotals] = useState<NetValues>({
        download: 0,
        upload: 0,
    });
    const [netMax, setNetMax] = useState(0);

    const [cpuUsage, setCpuUsage] = useState<CoreData[]>([]);

    const [ramData, setRamData] = useState<RamData>({
        available: 0,
        cached: 0,
        free: 0,
        total: 0,
        used: 0,
    });

    const [disks, setDisks] = useState([]);

    const [processes, setProcesses] = useState([]);

    const changeData = async () => {
        let newData = await (await fetch("/data")).json();
        if (netSpeeds.length > 30) netSpeeds.shift();

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

        setNetSpeeds([
            ...netSpeeds,
            {
                download: newData.net_stats.download_speed,
                upload: newData.net_stats.upload_speed,
            },
        ]);

        setNetMax(
            Math.max(
                ...netSpeeds.map((v) => v.download),
                ...netSpeeds.map((v) => v.upload)
            )
        );

        setNetTotals({
            download: newData.net_stats.download_total,
            upload: newData.net_stats.upload_total,
        });

        setTemp(Math.round(newData.temp));

        setCpuUsage(newData.cpu_usage);

        setRamData(newData.ram);

        setDisks(newData.disk || []);

        setProcesses(newData.proc);
    };

    useEffect(() => {
        if (!runOnce) {
            changeData();
            setRunOnce(true);
        }

        const interval = setInterval(changeData, 1000);
        return () => clearInterval(interval);
    });

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
                        netSpeeds={netSpeeds}
                        netMax={netMax}
                        netTotals={netTotals}
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
