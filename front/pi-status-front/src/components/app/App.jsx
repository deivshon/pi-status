import React from "react";
import { useState, useEffect } from 'react'

import Cpu from '../cpu/Cpu'
import Net from '../net/Net'
import Ram from '../ram/Ram'
import Proc from '../procs/Procs'

import 'bootstrap/dist/css/bootstrap.min.css'
import 'bootstrap/dist/js/bootstrap.bundle.min.js'
import './App.css'

export default function App() {
    const [netSpeeds, setNetSpeeds] = useState([])
    const [netTotals, setNetTotals] = useState({})
    const [netMax, setNetMax] = useState(0)
    const [ramData, setRamData] = useState({})
    const [processes, setProcesses] = useState([])

    const [temp, setTemp] = useState(0)
    const [cpuUsage, setCpuUsage] = useState([])

    const changeData = async () => {
        let newData = await (await fetch("/data")).json()
        if (netSpeeds.length > 30) netSpeeds.shift()

        setNetSpeeds([
            ...netSpeeds,
            {
                download: newData.net_stats.download_speed,
                upload: newData.net_stats.upload_speed
            }
        ])

        setNetMax(Math.max(
            ...(netSpeeds.map(v => v.download)),
            ...(netSpeeds.map(v => v.upload)))
        )

        setNetTotals({
            download: newData.net_stats.download_total,
            upload: newData.net_stats.upload_total,
        })
        
        setTemp(Math.round(newData.temp))

        setCpuUsage(newData.cpu_usage)

        setRamData(newData.ram)

        newData.proc.sort((p1, p2) => p1.mem - p2.mem)
        setProcesses(newData.proc.reverse())
    }

    useEffect(changeData, [])
    useEffect(() => {
        const interval = setInterval(changeData, 1000)
        return () => clearInterval(interval)
    })

    return (
        <div>
            <ul class="nav nav-pills mb-3 flex flex-wrap justify-content-center" id="pills-tab" role="tablist">
                <li class="nav-item fs-6 cpu-pill" role="presentation">
                    <button class="nav-link active" id="pills-cpu-tab" data-bs-toggle="pill" data-bs-target="#pills-cpu" type="button" role="tab">CPU</button>
                </li>
                <li class="nav-item fs-6 ram-pill" role="presentation">
                    <button class="nav-link" id="pills-ram-tab" data-bs-toggle="pill" data-bs-target="#pills-ram" type="button" role="tab">RAM</button>
                </li>
                <li class="nav-item fs-6 net-pill" role="presentation">
                    <button class="nav-link" id="pills-net-tab" data-bs-toggle="pill" data-bs-target="#pills-net" type="button" role="tab">Net</button>
                </li>
                <li class="nav-item fs-6 proc-pill" role="presentation">
                    <button class="nav-link" id="pills-proc-tab" data-bs-toggle="pill" data-bs-target="#pills-proc" type="button" role="tab">Ps</button>
                </li>
            </ul>
            <div class="tab-content w-100" id="pills-tabContent">
                <div class="tab-pane fade w-100 show active" id="pills-cpu" role="tabpanel">
                    <Cpu
                        temp={temp}
                        cpuUsage={cpuUsage}
                    />
                </div>
                <div class="tab-pane fade w-100" id="pills-ram" role="tabpanel">
                    <Ram
                        ramUsage={ramData}
                    />
                </div>
                <div class="tab-pane fade w-100" id="pills-net" role="tabpanel" aria-labelledby="pills-net-tab">
                    <Net
                        netSpeeds={netSpeeds}
                        netMax={netMax}
                        netTotals={netTotals}
                    />
                </div>
                <div class="tab-pane fade w-100" id="pills-proc" role="tabpanel">
                    <Proc
                        procs={processes}
                    />
                </div>
            </div>
        </div>
    );
}
